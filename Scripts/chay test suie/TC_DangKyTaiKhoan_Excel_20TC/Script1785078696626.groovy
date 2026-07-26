import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

// ================================
// 1. MỞ TRANG ĐĂNG KÝ TRỰC TIẾP
// ================================

WebUI.openBrowser('')

WebUI.navigateToUrl(
    'http://localhost/BanThuoc/public/khachHang/xacThuc/dangKy'
)

WebUI.delay(2)


// ================================
// 2. NHẬP DỮ LIỆU TỪ EXCEL
// ================================

WebUI.setText(
    findTestObject('Page_PharmaCare/input_Nguyn Vn A'),
    HoTen
)

WebUI.setText(
    findTestObject('Page_PharmaCare/input_examplegmail.com'),
    Email
)

WebUI.setText(
    findTestObject('Page_PharmaCare/input_09xx xxx xxx'),
    SoDienThoai
)

WebUI.setText(
    findTestObject('Page_PharmaCare/input_Ti thiu 6 k t'),
    MatKhau
)


// ================================
// 3. BẤM TẠO TÀI KHOẢN
// ================================

WebUI.click(
    findTestObject('Page_PharmaCare/button_To ti khon')
)


// Chờ website xử lý
WebUI.delay(3)


// ================================
// 4. KIỂM TRA ĐĂNG KÝ THÀNH CÔNG
// ================================

// Nếu đăng ký thành công → website chuyển sang trang đăng nhập
WebUI.verifyMatch(
    WebUI.getUrl(),
    '.*dangNhap.*',
    true
)


// ================================
// 5. ĐÓNG TRÌNH DUYỆT
// ================================

WebUI.closeBrowser()