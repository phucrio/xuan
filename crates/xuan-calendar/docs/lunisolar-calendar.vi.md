# Mô hình và triển khai lịch âm-dương

Tài liệu này mô tả thuật toán lịch âm-dương được triển khai trong `xuan-calendar`.
Đây là tài liệu nguyên bản của dự án, được viết lại dựa trên hành vi của crate. Nội dung
không phải bản sao hay bản dịch của tài liệu hoặc mã nguồn lịch từ bên thứ ba.

Bản tiếng Anh mặc định: [lunisolar-calendar.md](lunisolar-calendar.md)

## Phạm vi

`xuan-calendar` cung cấp các phép tính lịch thiên văn có tính xác định từ dữ liệu đầu vào
rõ ràng. Lớp lịch âm-dương hiện hỗ trợ:

- ngày dân sự Gregory theo lịch Gregory kéo dài ngược và xuôi thời gian (proleptic Gregorian);
- số ngày Julius và phép tính Julian Day Number;
- chuyển đổi âm-dương lịch Việt Nam và Trung Quốc bằng độ lệch UTC cố định;
- `TimeZone::VN` (`UTC+07:00`) và `TimeZone::CN` (`UTC+08:00`), đồng thời cho phép độ lệch cố định khác;
- xác định Trung khí để gán tháng nhuận;
- chuyển đổi hai chiều Gregorian ↔ lunisolar với kiểm tra round-trip.

Đây là một triển khai thiên văn/proleptic. Nó không tái dựng lịch pháp định trong lịch sử
và không nên được xem là nguồn thẩm quyền pháp lý hoặc lịch sử.

## Các quy tắc lịch được mô hình hóa

Triển khai tuân theo cấu trúc âm-dương lịch phổ biến trong lịch Việt Nam và Trung Quốc
hiện đại:

1. Tháng âm lịch bắt đầu vào ngày dân sự địa phương chứa thời điểm Sóc.
2. Năm thường có 12 tháng âm lịch; năm nhuận có 13 tháng.
3. Tháng 11 âm lịch được neo quanh Trung khí Đông chí.
4. Khi có 13 tháng âm giữa hai mốc tháng 11 liên tiếp, tháng đầu tiên không chứa Trung khí
   được chọn làm tháng nhuận.
5. Ngày dân sự chứa một sự kiện thiên văn phụ thuộc vào độ lệch UTC được truyền vào, nên
   cùng một thời điểm Sóc có thể bắt đầu tháng vào hai ngày khác nhau ở hai múi giờ.

Crate xử lý các quy tắc này bằng đầu vào tường minh, không phụ thuộc đồng hồ hệ thống,
cơ sở dữ liệu múi giờ, locale hay trạng thái hệ điều hành.

## Pipeline triển khai

### 1. Ngày Gregory sang JDN

`gregorian_to_jdn` ánh xạ `CivilDate` sang Julian Day Number nguyên bằng phép tính lịch
Gregory proleptic. Hàm nghịch đảo dựng lại ngày Gregory từ JDN.

Crate chủ động không chuyển sang lịch Julius đối với ngày trước cải cách Gregory. Vì vậy,
ngày lịch sử luôn được diễn giải nhất quán theo proleptic Gregorian.

### 2. Xấp xỉ thời điểm Sóc

`new_moon_jd(k)` tính một chuỗi thiên văn gọn cho chỉ số tuần trăng `k`. Chuỗi sử dụng
mốc Sóc trung bình, dị thường Mặt Trời, dị thường Mặt Trăng, đối số vĩ độ và các hiệu chỉnh
chu kỳ thường gặp trong nhóm công thức kiểu Meeus dùng cho tính lịch.

Kết quả là Julian Day theo thang thời gian thiên văn dạng UTC mà triển khai này sử dụng.
`new_moon_day_local` sau đó cộng độ lệch UTC cố định và xác định JDN địa phương chứa thời
điểm Sóc đó.

### 3. Kinh độ biểu kiến của Mặt Trời

`sun_longitude_rad` tính xấp xỉ kinh độ biểu kiến của Mặt Trời từ Julian Day. Hàm tính dị
thường và kinh độ trung bình, áp dụng phương trình tâm, sau đó thêm hiệu chỉnh nhỏ cho
kinh độ biểu kiến. Kết quả được chuẩn hóa về `[0, 2π)`.

Trung khí được biểu diễn bằng các ranh giới kinh độ Mặt Trời cách nhau 30 độ.

### 4. Mốc tháng 11

`lunar_month11_jdn` tìm ngày bắt đầu tháng theo Sóc gần cuối năm dương lịch và dùng sector
Trung khí để xác định mốc tháng 11. Hai mốc tháng 11 liên tiếp tạo thành khoảng năm âm
được dùng để đánh số tháng và xác định tháng nhuận.

### 5. Kiểm tra Trung khí trên khoảng tháng địa phương

Một tháng âm lịch được biểu diễn bằng khoảng nửa kín theo ngày địa phương:

```text
[start_of_month, start_of_next_month)
```

`month_has_principal_term_local` đổi hai biên địa phương sang Julian Day UTC rồi kiểm tra
xem kinh độ Mặt Trời có vượt qua ranh giới Trung khí 30 độ tiếp theo trong khoảng đó hay
không.

Điểm này quan trọng vì việc quyết định tháng nhuận được thực hiện trên toàn bộ khoảng
tháng dân sự địa phương, thay vì chỉ lấy một mẫu kinh độ tại thời điểm bắt đầu tháng.

### 6. Đếm tháng và gán tháng nhuận

Triển khai liệt kê các ngày Sóc địa phương giữa hai mốc tháng 11. Nếu có hơn 12 tháng,
`leap_month_offset` quét từng khoảng tháng địa phương và chọn khoảng đầu tiên không có
Trung khí.

Số tháng sau đó được đánh từ tháng 11 tiến về phía trước; tháng nhuận lặp lại số của tháng
thường ngay trước nó.

### 7. Chuyển đổi ngược

`lunar_to_gregorian` ưu tiên chiến lược dễ kiểm chứng: quét một khoảng JDN hữu hạn quanh
năm âm cần tìm, chuyển từng ngày Gregory ứng viên theo chiều thuận và trả về ngày có
`LunarDate` khớp chính xác với đầu vào.

Cách này kém tối ưu hơn công thức nghịch đảo trực tiếp về độ phức tạp, nhưng giúp chuyển
đổi ngược luôn đồng bộ với thuật toán thuận và tạo invariant round-trip mạnh cho test.

## Ranh giới reimplementation và provenance

Triển khai Rust công khai được duy trì như một **reimplementation** của các quy tắc lịch
và phương trình thiên văn đã được công bố. Repository **không** vendor hay phân phối lại
mã nguồn JavaScript, PHP, Java hoặc mã lịch của bên thứ ba, và cũng không chứa bảng âm
lịch dựng sẵn từ bên thứ ba.

Các phần sau được triển khai trực tiếp bằng Rust trong crate này:

- chuyển đổi và số học proleptic Gregorian/JDN;
- xấp xỉ thời điểm Sóc;
- xấp xỉ kinh độ biểu kiến Mặt Trời;
- quy đổi sự kiện sang ngày địa phương theo UTC offset cố định;
- xác định mốc tháng 11;
- kiểm tra Trung khí trên toàn khoảng tháng âm địa phương;
- liệt kê Sóc địa phương và xác định tháng nhuận;
- chuyển đổi ngược bằng kiểm chứng theo chiều thuận;
- kiểu dữ liệu, API, mô hình lỗi và regression tests riêng của crate.

Các quy tắc toán học, phương trình thiên văn và hệ số số học đã công bố chỉ đóng vai trò
tài liệu kỹ thuật cho phương pháp được triển khai. Văn phong, cấu trúc chương trình, biểu
đạt mã nguồn và dataset dựng sẵn của bên thứ ba không được tái cấp phép như một phần của
`xuan-calendar`.

Ranh giới này được ghi rõ có chủ đích: giấy phép của repository (`MIT OR Apache-2.0`) áp
dụng cho nội dung nguyên bản trong repository và không đưa ra tuyên bố nào về giấy phép
của các tài liệu tham khảo hoặc implementation bên ngoài.

Nếu contribution mới đưa mã, bảng dữ liệu hoặc dataset của bên thứ ba vào repository,
nguồn gốc và giấy phép phải được ghi rõ trước khi merge.

## Khác biệt so với nhóm implementation tham khảo của Hồ Ngọc Đức

Các bài viết và chương trình của Hồ Ngọc Đức là nguồn tham khảo hữu ích về quy tắc âm
lịch Việt Nam, nhưng `xuan-calendar` không phải bản port tương thích mã nguồn. Một số khác
biệt quan trọng:

| Hạng mục | `xuan-calendar` | Cách tiếp cận thường thấy trong tài liệu/implementation Hồ Ngọc Đức |
| --- | --- | --- |
| Lịch dân sự | Proleptic Gregorian cho toàn bộ miền ngày | Mã ví dụ có thể chuyển giữa lịch Julius và Gregory quanh năm 1582 |
| Múi giờ | UTC offset cố định theo phút | Ví dụ phổ biến dùng số giờ chênh lệch, đặc biệt Hà Nội `+7` |
| Sóc | Dùng trực tiếp chuỗi xấp xỉ gọn của crate | Một số routine tham khảo có thêm nhánh hiệu chỉnh ΔT |
| Trung khí | Kiểm tra ranh giới 30° trên toàn khoảng tháng địa phương | Routine gọn thường so sector kinh độ tại các mốc Sóc đầu tháng |
| Đếm tháng | Liệt kê trực tiếp các ngày Sóc địa phương | Routine gọn có thể suy ra số tháng từ chênh lệch số ngày |
| Chuyển đổi ngược | Quét hữu hạn và xác nhận bằng round-trip chiều thuận | Thường tính trực tiếp từ offset tháng và chỉ số Sóc |
| Dữ liệu lịch | Không dùng bảng âm lịch dựng sẵn | Một số bản JavaScript công bố dùng bảng tính sẵn trong một khoảng năm cố định |
| Lịch lịch sử | Không triển khai | Website Hồ Ngọc Đức có riêng lịch pháp định/lịch sử tái dựng |
| Can Chi | Được tích hợp ở các module khác của crate | Nằm ngoài phạm vi bài toán chuyển đổi âm-dương lịch cơ bản |

Các khác biệt này là lựa chọn thiết kế, không phải tuyên bố rằng thuật toán có độ chính
xác thiên văn cao hơn.

## Độ chính xác và kiểm chứng

Các hàm thiên văn hiện là xấp xỉ gọn, không phải ephemeris Mặt Trời/Mặt Trăng độ chính
xác cao. Những case sát biên có thể nhạy với sai số thời điểm nhỏ, nhất là khi Sóc hoặc
Trung khí rơi gần 00:00 theo giờ địa phương.

Regression suite chứa các case Việt Nam và Trung Quốc được chọn để kiểm tháng thường,
tháng nhuận, Tết, Can Chi và round-trip. Bộ test hiện có các vector chọn lọc từ năm 1800
đến 2620. Khoảng này chỉ mô tả **độ phủ regression**, không phải cam kết chính xác cho mọi
ngày trong toàn bộ khoảng thời gian đó.

Đối với nghiên cứu lịch sử, ngày rất xa thời hiện đại hoặc ứng dụng cần độ tin cậy cao ở
biên ngày, nên đối chiếu kết quả với ephemeris có thẩm quyền hoặc calendar oracle đáng tin
cậy.

Trong tương lai có thể thay lớp thiên văn xấp xỉ bằng ephemeris chính xác hơn mà vẫn giữ
nguyên mô hình quy tắc lịch và biên tháng địa phương của API.

## Hành vi liên quan trong crate

Thuật toán âm-dương lịch chỉ là một phần của `xuan-calendar`. Crate còn cung cấp:

- tiện ích Julian Day;
- chỉ số tiết khí;
- Can Chi năm/tháng/ngày/giờ;
- policy đổi ngày theo 23:00 giờ Tý hoặc 00:00 dân sự.

Các API này cùng dùng mô hình đầu vào xác định, nhưng không nên bị đồng nhất với quy tắc
đánh số tháng âm được mô tả ở trên.

## Tài liệu tham khảo

Các nguồn dưới đây được dùng để tham khảo khái niệm, kiểm chứng và đối chiếu. Việc liệt kê
không có nghĩa mã nguồn hay văn bản của các nguồn đó được phân phối theo giấy phép của
repository này.

- Hồ Ngọc Đức, *Thuật toán tính âm lịch*: https://www.xemamlich.uhm.vn/calrules.html
- Hồ Ngọc Đức, *Computing the Vietnamese lunar calendar*: https://www.xemamlich.uhm.vn/calrules_en.html
- Jean Meeus, *Astronomical Algorithms*, về các phương pháp xấp xỉ thiên văn tiêu chuẩn.
- Edward M. Reingold và Nachum Dershowitz, *Calendrical Calculations*, về thuật toán và thuật ngữ lịch.

Đối với implementation thực tế, nguồn chuẩn là mã Rust trong `src/lunar.rs`,
`src/julian.rs`, `src/solar.rs` và bộ kiểm thử `src/tests.rs`.
