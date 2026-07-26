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

WebUI.click(findTestObject('TC_QLXTTK_03/Page_PharmaCare/a_ng k ngay'))

WebUI.click(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_Nguyn Vn A'))

WebUI.setText(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_examplegmail.com'), 'tienminh.ds@gmail.com')

WebUI.setText(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_Nguyn Vn A'), 'Nguyễn')

WebUI.setText(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_09xx xxx xxx'), '0911234568')

WebUI.setEncryptedText(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_Ti thiu 6 k t'), '2S/pf5qdlLZ/x6e8cgByhw==')

WebUI.click(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_Ti thiu 6 k t'))

WebUI.click(findTestObject('TC_QLXTTK_03/Page_PharmaCare/button_To ti khon'))

WebUI.setEncryptedText(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_Ti thiu 6 k t'), 'gd5SMlrhfbTng9ZPiior2Q==')

WebUI.setText(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_examplegmail.com'), 'tienminh.ds@gmail.commm')

WebUI.click(findTestObject('TC_QLXTTK_03/Page_PharmaCare/button_To ti khon'))

WebUI.click(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_Nguyn Vn A'))

WebUI.setText(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_09xx xxx xxx'), '091123456834')

WebUI.setEncryptedText(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_Ti thiu 6 k t'), 'pmi2bqPJLQsu863V111Qkg==')

WebUI.click(findTestObject('TC_QLXTTK_03/Page_PharmaCare/input_Ti thiu 6 k t'))

WebUI.click(findTestObject('TC_QLXTTK_03/Page_PharmaCare/button_To ti khon'))

WebUI.rightClick(findTestObject('TC_QLXTTK_03/Page_PharmaCare/div_PharmaCare'))

WebUI.assertElementPresent(findTestObject('TC_QLXTTK_03/Page_PharmaCare/div_PharmaCare'), 0)

WebUI.closeBrowser()

