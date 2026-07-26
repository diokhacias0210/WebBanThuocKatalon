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

WebUI.setText(findTestObject('TC_QLTK_03/Page_PharmaCare/input_Nhp s in thoi'), '0902345678')

WebUI.click(findTestObject('TC_QLTK_03/Page_PharmaCare/input_Nhp mt khu'))

WebUI.doubleClick(findTestObject('TC_QLTK_03/Page_PharmaCare/input_Nhp mt khu'))

WebUI.setEncryptedText(findTestObject('TC_QLTK_03/Page_PharmaCare/input_Nhp mt khu'), 'aeHFOx8jV/A=')

WebUI.click(findTestObject('TC_QLTK_03/Page_PharmaCare/button_ng nhp'))

WebUI.click(findTestObject('TC_QLTK_03/Page_PharmaCare  Nh thuc trc tuyn/i_fa-solid fa-bars'))

WebUI.click(findTestObject('TC_QLTK_03/Page_PharmaCare  Nh thuc trc tuyn/a_Thng tin c nhn'))

WebUI.click(findTestObject('TC_QLTK_03/Page_PharmaCare  Thng tin c nhn/button_Sa'))

WebUI.doubleClick(findTestObject('TC_QLTK_03/Page_PharmaCare  Thng tin c nhn/button_Sa'))

WebUI.click(findTestObject('TC_QLTK_03/Page_PharmaCare  Thng tin c nhn/button_Sa_1'))

WebUI.doubleClick(findTestObject('TC_QLTK_03/Page_PharmaCare  Thng tin c nhn/button_Sa_1'))

WebUI.click(findTestObject('TC_QLTK_03/Page_PharmaCare  Thng tin c nhn/button_Sa_2'))

WebUI.doubleClick(findTestObject('TC_QLTK_03/Page_PharmaCare  Thng tin c nhn/button_Sa_2'))

WebUI.click(findTestObject('TC_QLTK_03/Page_PharmaCare  Thng tin c nhn/button_Sa_2'))

WebUI.rightClick(findTestObject('TC_QLTK_03/Page_PharmaCare  Thng tin c nhn/div_a ch giao hng'))

WebUI.assertElementPresent(findTestObject('TC_QLTK_03/Page_PharmaCare  Thng tin c nhn/div_a ch giao hng'), 0)

WebUI.closeBrowser()

