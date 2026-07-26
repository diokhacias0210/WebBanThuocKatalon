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

WebUI.setText(findTestObject('TC_QLTK_05/Page_PharmaCare/input_Nhp s in thoi'), '0913456789')

WebUI.setEncryptedText(findTestObject('TC_QLTK_05/Page_PharmaCare/input_Nhp mt khu'), 'aeHFOx8jV/A=')

WebUI.click(findTestObject('TC_QLTK_05/Page_PharmaCare/button_ng nhp'))

WebUI.click(findTestObject('TC_QLTK_05/Page_PharmaCare  PharmaCare  H s dc s/button_btnEditProfile'))

WebUI.setText(findTestObject('TC_QLTK_05/Page_PharmaCare  PharmaCare  H s dc s/input_f_hoTen'), 'Dược sĩ Phạm ')

WebUI.setText(findTestObject('TC_QLTK_05/Page_PharmaCare  PharmaCare  H s dc s/input_f_soDienThoai'), '091345678')

WebUI.click(findTestObject('TC_QLTK_05/Page_PharmaCare  PharmaCare  H s dc s/button_btnModalCancel'))

WebUI.rightClick(findTestObject('TC_QLTK_05/Page_PharmaCare  PharmaCare  H s dc s/div_divider'))

WebUI.assertElementPresent(findTestObject('TC_QLTK_05/Page_PharmaCare  PharmaCare  H s dc s/div_divider'), 0)

WebUI.closeBrowser()

