import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser(null)

WebUI.navigateToUrl('http://localhost/BanThuoc/public/khachHang/xacThuc/dangNhap')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('TC_QLTK_08/Page_PharmaCare/input_Nhp s in thoi'), '0999888777')

WebUI.setEncryptedText(findTestObject('TC_QLTK_08/Page_PharmaCare/input_Nhp mt khu'), 'aeHFOx8jV/A=')

WebUI.click(findTestObject('TC_QLTK_08/Page_PharmaCare/button_ng nhp'))

WebUI.click(findTestObject('TC_QLTK_08/Page_Admin Panel  Qun L Ti Khon/i_fa-solid fa-sliders'))

WebUI.selectOptionByValue(findTestObject('TC_QLTK_08/Page_Admin Panel  Qun L Ti Khon/select_f_role_select'), 'KHACH_HANG', 
    false)

WebUI.click(findTestObject('TC_QLTK_08/Page_Admin Panel  Qun L Ti Khon/button_btnSaveRole'))

WebUI.rightClick(findTestObject('TC_QLTK_08/Page_Admin Panel  Qun L Ti Khon/td_Hot ng'))

WebUI.assertElementPresent(findTestObject('TC_QLTK_08/Page_Admin Panel  Qun L Ti Khon/td_Hot ng'), 0)

WebUI.rightClick(findTestObject('TC_QLTK_08/Page_Admin Panel  Qun L Ti Khon/section_Tt c vai tr'))

WebUI.assertElementPresent(findTestObject('TC_QLTK_08/Page_Admin Panel  Qun L Ti Khon/section_Tt c vai tr'), 0)

WebUI.closeBrowser()

