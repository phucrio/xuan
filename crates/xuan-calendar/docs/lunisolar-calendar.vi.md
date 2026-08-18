# Mô hình và cách triển khai âm dương lịch

Tài liệu này mô tả mô hình âm dương lịch được `xuan-calendar` triển khai, gồm các quy tắc
lịch pháp, phép xấp xỉ thiên văn, cách xử lý múi giờ và những lựa chọn kỹ thuật chính.

Bản tiếng Anh mặc định: [lunisolar-calendar.md](lunisolar-calendar.md)

## Phạm vi

`xuan-calendar` thực hiện các phép tính lịch theo cách xác định hoàn toàn từ dữ liệu đầu
vào. Phần âm dương lịch hiện hỗ trợ:

- ngày theo lịch Gregory kéo dài (proleptic Gregorian);
- Ngày Julius (Julian Day, JD) và Số ngày Julius (Julian Day Number, JDN);
- chuyển đổi giữa dương lịch và âm dương lịch Việt Nam/Trung Quốc với độ lệch UTC được
  chỉ định rõ;
- các múi giờ dựng sẵn `TimeZone::VN` (`UTC+07:00`), `TimeZone::CN` (`UTC+08:00`) và các
  độ lệch UTC cố định khác;
- xác định Trung khí để tìm tháng nhuận;
- chuyển đổi hai chiều Gregorian ↔ lunisolar và kiểm tra tính nhất quán bằng round-trip.

Mô hình hiện tại là mô hình âm dương lịch thiên văn. Việc phục dựng lịch pháp lịch sử
không thuộc phạm vi của crate ở thời điểm này.

## Các quy tắc lịch pháp

Phần âm dương lịch tuân theo các quy tắc chính sau:

1. Mỗi tháng âm bắt đầu vào ngày địa phương chứa thời điểm Sóc.
2. Năm thường có 12 tháng âm, năm nhuận có 13 tháng.
3. Tháng 11 âm lịch là tháng chứa Trung khí Đông chí.
4. Nếu giữa hai mốc tháng 11 liên tiếp có 13 tháng âm, tháng đầu tiên không chứa Trung khí
   được chọn làm tháng nhuận.
5. Ngày địa phương chứa một sự kiện thiên văn phụ thuộc vào độ lệch UTC. Vì vậy cùng một
   thời điểm Sóc có thể rơi vào hai ngày lịch khác nhau ở hai múi giờ khác nhau.

Các quy tắc trên được tính trực tiếp từ đầu vào, không phụ thuộc vào đồng hồ hệ thống hay
thiết lập locale của máy chạy.

## Các bước triển khai

### 1. Chuyển ngày Gregory sang JDN

`gregorian_to_jdn` chuyển `CivilDate` thành JDN nguyên bằng phép tính trên lịch Gregory
kéo dài. Hàm nghịch đảo dựng lại ngày Gregory từ JDN.

Crate dùng proleptic Gregorian cho toàn bộ miền ngày, thay vì chuyển sang lịch Julius ở
các ngày trước cuộc cải cách lịch Gregory.

### 2. Xấp xỉ thời điểm Sóc

`new_moon_jd(k)` tính gần đúng thời điểm Sóc của tuần trăng thứ `k` bằng một chuỗi thiên
văn gọn. Công thức sử dụng mốc Sóc trung bình, dị thường của Mặt Trời và Mặt Trăng, đối số
vĩ độ của Mặt Trăng cùng các số hạng hiệu chỉnh tuần hoàn thường gặp trong nhóm phương
pháp kiểu Meeus dùng cho tính lịch.

`new_moon_day_local` sau đó áp dụng độ lệch UTC và xác định ngày địa phương chứa thời điểm
Sóc vừa tính được.

### 3. Kinh độ biểu kiến của Mặt Trời

`sun_longitude_rad` xấp xỉ kinh độ biểu kiến của Mặt Trời tại một Julian Day. Hàm tính dị
thường trung bình và kinh độ trung bình của Mặt Trời, áp dụng phương trình tâm cùng một
hiệu chỉnh nhỏ cho kinh độ biểu kiến, rồi chuẩn hóa kết quả về `[0, 2π)`.

Trong mô hình này, các Trung khí tương ứng với những ranh giới kinh độ Mặt Trời cách nhau
30 độ.

### 4. Xác định mốc tháng 11

`lunar_month11_jdn` tìm ngày Sóc gần cuối năm dương lịch, sau đó dựa vào phân đoạn kinh độ
Mặt Trời để xác định Sóc bắt đầu tháng 11. Hai mốc tháng 11 liên tiếp tạo thành khoảng
tham chiếu dùng để đánh số tháng và xác định năm có tháng nhuận hay không.

### 5. Kiểm tra Trung khí trong từng tháng âm

Mỗi tháng âm được biểu diễn bằng một khoảng nửa mở theo ngày địa phương:

```text
[start_of_month, start_of_next_month)
```

`month_has_principal_term_local` đổi hai đầu mút của khoảng này sang Julian Day theo UTC,
rồi kiểm tra xem kinh độ Mặt Trời có vượt qua ranh giới Trung khí 30 độ kế tiếp trong
khoảng đó hay không.

Cách kiểm tra trên toàn khoảng tháng đặc biệt hữu ích ở những trường hợp sát biên ngày
hoặc biên múi giờ, nơi việc chỉ lấy một giá trị kinh độ tại đầu tháng có thể cho kết quả
không ổn định.

### 6. Đếm tháng và xác định tháng nhuận

Triển khai liệt kê trực tiếp các ngày Sóc địa phương giữa hai mốc tháng 11. Nếu khoảng đó
có 13 tháng âm, `leap_month_offset` duyệt từng tháng và chọn tháng đầu tiên không chứa
Trung khí làm tháng nhuận.

Việc đánh số tháng bắt đầu từ tháng 11 và tiến tuần tự. Tháng nhuận giữ cùng số tháng với
tháng thường đứng ngay trước nó.

### 7. Chuyển ngược từ âm lịch sang dương lịch

`lunar_to_gregorian` dùng một chiến lược kiểm chứng hữu hạn: quét một khoảng JDN quanh
năm âm cần tìm, chuyển từng ngày dương lịch ứng viên sang âm lịch, rồi trả về ngày có
`LunarDate` khớp chính xác với đầu vào.

Cách làm này ưu tiên việc giữ thuật toán nghịch đảo nhất quán tuyệt đối với thuật toán
thuận, thay vì duy trì thêm một công thức nghịch đảo độc lập. Đồng thời, nó tạo ra một
điều kiện round-trip rõ ràng cho bộ kiểm thử hồi quy.

## Các thành phần được tái triển khai

Các phần sau được triển khai trực tiếp bằng Rust trong `xuan-calendar`:

- chuyển đổi giữa lịch Gregory kéo dài và JDN;
- cộng/trừ ngày thông qua JDN;
- xấp xỉ thời điểm Sóc;
- xấp xỉ kinh độ biểu kiến của Mặt Trời;
- xác định ngày địa phương của sự kiện thiên văn từ độ lệch UTC;
- xác định mốc tháng 11;
- kiểm tra Trung khí trên toàn khoảng tháng âm;
- liệt kê các ngày Sóc và xác định tháng nhuận;
- chuyển đổi ngược bằng cách kiểm chứng qua chiều thuận;
- các kiểu dữ liệu, API, xử lý lỗi và bộ kiểm thử hồi quy của crate.

Các quy tắc lịch pháp và công thức thiên văn đã được công bố được dùng làm cơ sở kỹ thuật
để xây dựng các phép tính trên. Phần Rust được tổ chức theo mô hình dữ liệu, cách biểu
diễn khoảng tháng địa phương và API riêng của `xuan-calendar`.

## Khác biệt so với các triển khai tham khảo phổ biến của Hồ Ngọc Đức

Các bài viết và chương trình của Hồ Ngọc Đức là tài liệu tham khảo quan trọng cho quy tắc
âm lịch Việt Nam. `xuan-calendar` giữ cùng nền tảng lịch pháp nhưng có một số lựa chọn
triển khai khác:

| Hạng mục | `xuan-calendar` | Cách làm thường thấy trong tài liệu tham khảo |
| --- | --- | --- |
| Lịch dương | Dùng proleptic Gregorian cho toàn bộ miền ngày | Mã ví dụ có thể chuyển giữa lịch Julius và Gregory quanh năm 1582 |
| Múi giờ | Dùng độ lệch UTC cố định tính bằng phút | Ví dụ phổ biến thường truyền độ lệch theo giờ |
| Tính Sóc | Dùng trực tiếp chuỗi xấp xỉ gọn của crate | Một số routine có thêm nhánh hiệu chỉnh ΔT |
| Xác định Trung khí | Kiểm tra việc vượt ranh giới 30° trên toàn khoảng tháng địa phương | Routine gọn thường so phân đoạn kinh độ tại các mốc Sóc |
| Đếm tháng | Liệt kê trực tiếp các ngày Sóc địa phương | Một số routine suy ra số tháng từ chênh lệch số ngày |
| Chuyển đổi ngược | Tìm trong khoảng hữu hạn rồi xác nhận lại bằng chiều thuận | Thường tính trực tiếp từ offset tháng và chỉ số Sóc |
| Dữ liệu lịch | Tính từ công thức tại thời điểm chạy | Một số triển khai còn cung cấp bảng tính sẵn cho một khoảng năm cố định |
| Lịch pháp lịch sử | Chưa thuộc phạm vi hiện tại | Có thể được xây dựng thành một lớp phục dựng riêng |
| Can Chi | Được tích hợp trong các phần khác của crate | Thường tách khỏi thuật toán chuyển đổi âm dương lịch cơ bản |

Đây là khác biệt về cách tổ chức và triển khai thuật toán, không phải tuyên bố rằng
`xuan-calendar` có độ chính xác thiên văn cao hơn các nguồn tham khảo.

## Độ chính xác và kiểm chứng

Các hàm thiên văn hiện dùng những công thức xấp xỉ gọn, không phải ephemeris Mặt Trời và
Mặt Trăng độ chính xác cao. Những trường hợp nằm sát biên ngày có thể nhạy với sai số thời
gian nhỏ, đặc biệt khi Sóc hoặc Trung khí rơi gần 00:00 theo giờ địa phương.

Bộ kiểm thử hồi quy có các trường hợp Việt Nam và Trung Quốc được chọn để kiểm tra tháng
thường, tháng nhuận, ngày đầu năm âm lịch, Can Chi và tính round-trip. Hiện các ca kiểm thử
bao phủ một số mốc từ năm 1800 đến 2620. Đây là phạm vi kiểm thử, không phải cam kết rằng
mọi ngày trong khoảng này đều đạt cùng một mức sai số thiên văn.

Với nghiên cứu lịch sử, ngày ở rất xa thời hiện đại hoặc ứng dụng phụ thuộc chính xác vào
thời điểm chuyển ngày, nên đối chiếu kết quả với ephemeris phù hợp hoặc một nguồn lịch
đáng tin cậy.

Lớp tính toán thiên văn có thể được thay bằng mô hình chính xác hơn trong tương lai mà
không cần thay đổi mô hình tháng địa phương ở API công khai.

## Các chức năng liên quan

Thuật toán âm dương lịch chỉ là một phần của `xuan-calendar`. Crate còn cung cấp:

- các hàm tiện ích Julian Day;
- chỉ số tiết khí;
- tính Can Chi năm, tháng, ngày và giờ;
- lựa chọn mốc đổi ngày lúc 23:00 giờ Tý hoặc 00:00 theo ngày dân sự.

## Tài liệu tham khảo

Các nguồn tham khảo chính cho quy tắc lịch pháp, thuật ngữ, phương pháp thiên văn và việc
đối chiếu kết quả:

- Hồ Ngọc Đức, *Thuật toán tính âm lịch*: https://www.xemamlich.uhm.vn/calrules.html
- Hồ Ngọc Đức, *Computing the Vietnamese lunar calendar*: https://www.xemamlich.uhm.vn/calrules_en.html
- Jean Meeus, *Astronomical Algorithms*.
- Edward M. Reingold và Nachum Dershowitz, *Calendrical Calculations*.

Chi tiết triển khai nằm trong `src/lunar.rs`, `src/julian.rs`, `src/solar.rs` và
`src/tests.rs`.