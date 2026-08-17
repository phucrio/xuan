# Mô hình và triển khai lịch âm-dương

Tài liệu này mô tả mô hình lịch âm-dương được triển khai trong `xuan-calendar`, gồm các
xấp xỉ thiên văn, quy tắc theo giờ địa phương và các lựa chọn triển khai chính.

Bản tiếng Anh mặc định: [lunisolar-calendar.md](lunisolar-calendar.md)

## Phạm vi

`xuan-calendar` cung cấp các phép tính lịch có tính xác định từ đầu vào tường minh. Lớp
lịch âm-dương hiện hỗ trợ:

- ngày dân sự Gregory theo mô hình proleptic Gregorian;
- số ngày Julius và phép tính Julian Day Number;
- chuyển đổi âm-dương lịch Việt Nam và Trung Quốc bằng UTC offset cố định;
- `TimeZone::VN` (`UTC+07:00`) và `TimeZone::CN` (`UTC+08:00`), cùng các offset cố định khác;
- xác định Trung khí để gán tháng nhuận;
- chuyển đổi hai chiều Gregorian ↔ lunisolar với kiểm tra round-trip.

Triển khai hiện mô hình hóa lịch âm-dương theo phương pháp thiên văn. Tái dựng lịch lịch
sử không nằm trong phạm vi hiện tại.

## Các quy tắc lịch

Mô hình sử dụng các quy tắc sau:

1. Tháng âm lịch bắt đầu vào ngày dân sự địa phương chứa thời điểm Sóc.
2. Năm thường có 12 tháng âm lịch; năm nhuận có 13 tháng.
3. Tháng 11 âm lịch được neo quanh Trung khí Đông chí.
4. Khi có 13 tháng âm giữa hai mốc tháng 11 liên tiếp, tháng đầu tiên không chứa Trung khí
   được chọn làm tháng nhuận.
5. Ngày dân sự chứa một sự kiện thiên văn phụ thuộc vào UTC offset được truyền vào, nên
   cùng một thời điểm Sóc có thể bắt đầu tháng vào hai ngày khác nhau ở hai múi giờ.

Crate tính các quy tắc này từ dữ liệu đầu vào thay vì phụ thuộc vào đồng hồ hoặc locale
của máy chạy.

## Pipeline triển khai

### 1. Ngày Gregory và JDN

`gregorian_to_jdn` ánh xạ `CivilDate` sang Julian Day Number nguyên bằng số học
proleptic Gregorian. Hàm nghịch đảo dựng lại ngày Gregory từ JDN.

Triển khai dùng proleptic Gregorian xuyên suốt thay vì chuyển sang lịch Julius quanh mốc
cải cách Gregory lịch sử.

### 2. Xấp xỉ thời điểm Sóc

`new_moon_jd(k)` tính một chuỗi thiên văn gọn cho chỉ số tuần trăng `k`. Chuỗi sử dụng
mốc Sóc trung bình, dị thường Mặt Trời, dị thường Mặt Trăng, đối số vĩ độ và các hiệu chỉnh
chu kỳ thường gặp trong nhóm công thức kiểu Meeus dùng cho tính lịch.

`new_moon_day_local` sau đó áp UTC offset cố định và ánh xạ thời điểm Sóc sang ngày dân
sự địa phương chứa sự kiện đó.

### 3. Kinh độ biểu kiến của Mặt Trời

`sun_longitude_rad` tính xấp xỉ kinh độ biểu kiến của Mặt Trời từ Julian Day. Hàm tính dị
thường và kinh độ trung bình, áp dụng phương trình tâm và một hiệu chỉnh nhỏ cho kinh độ
biểu kiến, rồi chuẩn hóa kết quả về `[0, 2π)`.

Trung khí được biểu diễn bằng các ranh giới kinh độ Mặt Trời cách nhau 30 độ.

### 4. Mốc tháng 11

`lunar_month11_jdn` tìm ngày bắt đầu tháng theo Sóc gần cuối năm dương lịch và dùng sector
Trung khí để chọn mốc tháng 11. Hai mốc tháng 11 liên tiếp tạo thành khoảng dùng để đánh
số tháng và xác định tháng nhuận.

### 5. Kiểm tra Trung khí trên khoảng tháng địa phương

Một tháng âm lịch được biểu diễn bằng khoảng nửa kín theo ngày địa phương:

```text
[start_of_month, start_of_next_month)
```

`month_has_principal_term_local` đổi hai biên địa phương sang Julian Day UTC rồi kiểm tra
xem kinh độ Mặt Trời có vượt qua ranh giới Trung khí 30 độ tiếp theo trong khoảng đó hay
không.

Cách kiểm tra theo toàn khoảng tháng hữu ích ở các case sát biên múi giờ hoặc biên ngày,
nơi việc chỉ lấy một mẫu kinh độ tại đầu tháng có thể gây mơ hồ.

### 6. Đếm tháng và gán tháng nhuận

Triển khai liệt kê các ngày Sóc địa phương giữa hai mốc tháng 11. Nếu có 13 tháng âm,
`leap_month_offset` quét từng khoảng tháng địa phương và chọn khoảng đầu tiên không có
Trung khí.

Số tháng được đánh từ tháng 11 tiến về phía trước; tháng nhuận lặp lại số của tháng thường
ngay trước nó.

### 7. Chuyển đổi ngược

`lunar_to_gregorian` dùng chiến lược kiểm chứng hữu hạn: quét một khoảng JDN quanh năm âm
cần tìm, chuyển từng ngày Gregory ứng viên theo chiều thuận và trả về ngày có `LunarDate`
khớp chính xác với đầu vào.

Cách này ưu tiên sự nhất quán với thuật toán thuận hơn việc duy trì một công thức nghịch
đảo riêng, đồng thời tạo invariant round-trip rõ ràng cho regression tests.

## Các phần được reimplement

Crate triển khai trực tiếp bằng Rust các thành phần sau:

- chuyển đổi và số học proleptic Gregorian/JDN;
- số học ngày Gregory thông qua JDN;
- xấp xỉ thời điểm Sóc;
- xấp xỉ kinh độ biểu kiến Mặt Trời;
- quy đổi sự kiện sang ngày địa phương theo UTC offset cố định;
- xác định mốc tháng 11;
- kiểm tra Trung khí trên toàn khoảng tháng âm địa phương;
- liệt kê Sóc địa phương và xác định tháng nhuận;
- chuyển đổi ngược bằng kiểm chứng theo chiều thuận;
- kiểu dữ liệu, API, xử lý lỗi và regression tests của crate.

Các quy tắc lịch và công thức thiên văn đã công bố được dùng làm tài liệu kỹ thuật cho
những phép tính này. Triển khai Rust được tổ chức theo data model, logic khoảng tháng địa
phương và API riêng của `xuan-calendar`.

## Khác biệt so với nhóm implementation tham khảo của Hồ Ngọc Đức

Các bài viết và chương trình của Hồ Ngọc Đức là nguồn tham khảo hữu ích về quy tắc âm
lịch Việt Nam. `xuan-calendar` có một số khác biệt triển khai đáng chú ý:

| Hạng mục | `xuan-calendar` | Cách tiếp cận tham khảo thường gặp |
| --- | --- | --- |
| Lịch dân sự | Proleptic Gregorian cho toàn bộ miền ngày | Mã ví dụ có thể chuyển giữa lịch Julius và Gregory quanh năm 1582 |
| Múi giờ | UTC offset cố định theo phút | Ví dụ phổ biến dùng số giờ chênh lệch |
| Sóc | Dùng trực tiếp chuỗi xấp xỉ gọn của crate | Một số routine có thêm nhánh hiệu chỉnh ΔT |
| Trung khí | Kiểm tra ranh giới 30° trên toàn khoảng tháng địa phương | Routine gọn thường so sector kinh độ tại các mốc Sóc đầu tháng |
| Đếm tháng | Liệt kê trực tiếp các ngày Sóc địa phương | Routine gọn có thể suy ra số tháng từ chênh lệch số ngày |
| Chuyển đổi ngược | Quét hữu hạn và xác nhận bằng round-trip chiều thuận | Thường tính trực tiếp từ offset tháng và chỉ số Sóc |
| Dữ liệu lịch | Tính từ công thức khi chạy | Một số implementation cũng cung cấp bảng dựng sẵn cho một khoảng năm cố định |
| Lịch lịch sử | Ngoài phạm vi hiện tại | Có thể được xử lý bằng mô hình tái dựng riêng |
| Can Chi | Được tích hợp ở các module khác của crate | Thường tách khỏi bài toán chuyển đổi âm-dương lịch cơ bản |

Các khác biệt này là lựa chọn thiết kế, không phải tuyên bố rằng thuật toán có độ chính
xác thiên văn cao hơn.

## Độ chính xác và kiểm chứng

Các hàm thiên văn hiện là xấp xỉ gọn, không phải ephemeris Mặt Trời hoặc Mặt Trăng độ
chính xác cao. Những case sát biên có thể nhạy với sai số thời điểm nhỏ, nhất là khi Sóc
hoặc Trung khí rơi gần 00:00 theo giờ địa phương.

Regression suite chứa các case Việt Nam và Trung Quốc được chọn để kiểm tháng thường,
tháng nhuận, Tết, Can Chi và round-trip. Bộ test hiện có các vector chọn lọc từ năm 1800
đến 2620. Khoảng này mô tả độ phủ regression, không phải cam kết độ chính xác cho mọi ngày
trong toàn bộ khoảng thời gian đó.

Đối với nghiên cứu lịch sử, ngày rất xa thời hiện đại hoặc ứng dụng phụ thuộc vào thời
điểm sát biên ngày, nên đối chiếu kết quả với ephemeris phù hợp hoặc calendar oracle đáng
tin cậy.

Trong tương lai có thể thay lớp thiên văn xấp xỉ bằng implementation chính xác hơn mà vẫn
giữ nguyên mô hình khoảng tháng địa phương của public API.

## Hành vi liên quan trong crate

Thuật toán âm-dương lịch chỉ là một phần của `xuan-calendar`. Crate còn cung cấp:

- tiện ích Julian Day;
- chỉ số tiết khí;
- Can Chi năm/tháng/ngày/giờ;
- policy đổi ngày theo 23:00 giờ Tý hoặc 00:00 dân sự.

## Tài liệu tham khảo

Các nguồn dùng để tham khảo khái niệm, thuật ngữ, kiểm chứng và đối chiếu:

- Hồ Ngọc Đức, *Thuật toán tính âm lịch*: https://www.xemamlich.uhm.vn/calrules.html
- Hồ Ngọc Đức, *Computing the Vietnamese lunar calendar*: https://www.xemamlich.uhm.vn/calrules_en.html
- Jean Meeus, *Astronomical Algorithms*.
- Edward M. Reingold và Nachum Dershowitz, *Calendrical Calculations*.

Chi tiết triển khai nằm trong `src/lunar.rs`, `src/julian.rs`, `src/solar.rs` và
`src/tests.rs`.
