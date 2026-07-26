import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

// ============================================================
// TEST CASE: Thêm địa chỉ giao hàng - chạy dữ liệu từ Excel
// Excel: Data Files/TestData_ThemDiaChi
// ============================================================

// Lấy Test Data từ Excel
TestData data = TestDataFactory.findTestData('Data Files/TestData_ThemDiaChi')

// Lấy dòng hiện tại của Test Suite/Test Case
int row = data.getRowIndex()

// Nếu chạy trực tiếp Test Case, mặc định chạy từ dòng 1
if (row < 1) {
    row = 1
}

// Đọc dữ liệu Excel
String tcId           = data.getValue('TC_ID', row)
String nhanTenDiaChi  = data.getValue('NhanTenDiaChi', row)
String tenNguoiNhan   = data.getValue('TenNguoiNhan', row)
String soDienThoai    = data.getValue('SoDienThoai', row)
String diaChiDayDu    = data.getValue('DiaChiDayDu', row)
String ghiChu         = data.getValue('GhiChu', row)
String macDinh        = data.getValue('MacDinh', row)

println("========== ĐANG CHẠY: ${tcId} ==========")
println("Nhãn địa chỉ: ${nhanTenDiaChi}")
println("Người nhận: ${tenNguoiNhan}")
println("SĐT: ${soDienThoai}")
println("Địa chỉ: ${diaChiDayDu}")
println("Mặc định: ${macDinh}")

// ============================================================
// 1. MỞ TRÌNH DUYỆT + ĐĂNG NHẬP
// ============================================================

WebUI.openBrowser('')

WebUI.navigateToUrl('http://localhost/BanThuoc/public/khachHang/xacThuc/dangNhap')

WebUI.setText(
    findTestObject('Page_PharmaCare/input_Nhp s in thoi'),
    '0902345678'
)

WebUI.setEncryptedText(
    findTestObject('Page_PharmaCare/input_Nhp mt khu'),
    'aeHFOx8jV/A='
)

WebUI.click(
    findTestObject('Page_PharmaCare/button_ng nhp')
)

// ============================================================
// 2. VÀO THÔNG TIN CÁ NHÂN -> THÊM ĐỊA CHỈ
// ============================================================

WebUI.click(
    findTestObject('Page_PharmaCare  Nh thuc trc tuyn/i_fa-solid fa-bars')
)

WebUI.click(
    findTestObject('Page_PharmaCare  Nh thuc trc tuyn/a_Thng tin c nhn')
)

WebUI.click(
    findTestObject('Page_PharmaCare  Thng tin c nhn/button_Thm a')
)

// ============================================================
// 3. NHẬP DỮ LIỆU TỪ EXCEL
// ============================================================

// Nhãn tên địa chỉ
WebUI.setText(
    findTestObject('Page_PharmaCare  Thng tin c nhn/input_VD_ Nh ring, C quan, Nh ni, Kho hng'),
    nhanTenDiaChi
)

// Tên người nhận
WebUI.setText(
    findTestObject('Page_PharmaCare  Thng tin c nhn/input_Nguyen Van An'),
    tenNguoiNhan
)

// Số điện thoại
WebUI.setText(
    findTestObject('Page_PharmaCare  Thng tin c nhn/input_0912 345 678'),
    soDienThoai
)

// Địa chỉ giao hàng đầy đủ
WebUI.setText(
    findTestObject('Page_PharmaCare  Thng tin c nhn/input_S nh, tn ng, phng, qun, tnh thn'),
    diaChiDayDu
)

// Ghi chú
WebUI.setText(
    findTestObject('Page_PharmaCare  Thng tin c nhn/textarea_VD_ Giao gi hnh chnh, gi trc 15 p'),
    ghiChu
)

// Đặt làm địa chỉ mặc định nếu Excel ghi "Có"
if (macDinh.equalsIgnoreCase('Có') || macDinh.equalsIgnoreCase('Yes')) {
    WebUI.click(
        findTestObject('Page_PharmaCare  Thng tin c nhn/label_t lm a ch mc nh ngay khi to')
    )
}

// ============================================================
// 4. LƯU ĐỊA CHỈ
// ============================================================

WebUI.click(
    findTestObject('Page_PharmaCare  Thng tin c nhn/button_Lu a ch')
)

// ============================================================
// 5. KIỂM TRA KẾT QUẢ
// ============================================================

TestObject diaChiMoi = findTestObject(
    'Page_PharmaCare  Thng tin c nhn/div_a ch giao hng_1'
)

WebUI.verifyElementPresent(
    diaChiMoi,
    10,
    FailureHandling.STOP_ON_FAILURE
)

println("========== ${tcId} PASS ==========")

WebUI.closeBrowser()
