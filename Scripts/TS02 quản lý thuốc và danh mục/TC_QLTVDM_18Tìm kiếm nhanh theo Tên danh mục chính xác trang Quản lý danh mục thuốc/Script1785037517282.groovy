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

WebUI.navigateToUrl('http://localhost/BanThuoc/public/')

WebUI.maximizeWindow()

WebUI.click(findTestObject('TC_QLTVDM_18/Page_PharmaCare  Nh thuc trc tuyn/i_fa-solid fa-bars'))

WebUI.click(findTestObject('TC_QLTVDM_18/Page_PharmaCare  Nh thuc trc tuyn/a_ng nhp'))

WebUI.click(findTestObject('TC_QLTVDM_18/Page_PharmaCare/input_Nhp s in thoi'))

WebUI.doubleClick(findTestObject('TC_QLTVDM_18/Page_PharmaCare/input_Nhp s in thoi'))

WebUI.click(findTestObject('TC_QLTVDM_18/Page_PharmaCare/input_Nhp mt khu'))

WebUI.click(findTestObject('TC_QLTVDM_18/Page_PharmaCare/form_S in thoi'))

WebUI.setText(findTestObject('TC_QLTVDM_18/Page_PharmaCare/input_Nhp s in thoi'), '0999888777')

WebUI.setEncryptedText(findTestObject('TC_QLTVDM_18/Page_PharmaCare/input_Nhp mt khu'), 'aeHFOx8jV/A=')

WebUI.click(findTestObject('TC_QLTVDM_18/Page_PharmaCare/button_ng nhp'))

WebUI.click(findTestObject('TC_QLTVDM_18/Page_Admin Panel  Qun L Ti Khon/a_Qun l danh mc thuc'))

WebUI.setText(findTestObject('TC_QLTVDM_18/Page_Admin Panel  Qun L Danh Mc Thuc/input_Tm nhanh theo tn danh mc'), '')

WebUI.click(findTestObject('TC_QLTVDM_18/Page_Admin Panel  Qun L Danh Mc Thuc/a_Qun l thuc'))

WebUI.setText(findTestObject('TC_QLTVDM_18/Page_Admin Panel  Qun L Thuc/input_Tm kim nhanh tn thuc hoc hot cht'), 'bột ')

WebUI.click(findTestObject('TC_QLTVDM_18/Page_Admin Panel  Qun L Thuc/button_btnResetFilter'))

WebUI.rightClick(findTestObject('TC_QLTVDM_18/Page_Admin Panel  Qun L Thuc/div_Tt c danh mcCha phn loiThuc gim au'))

WebUI.assertElementPresent(findTestObject('TC_QLTVDM_18/Page_Admin Panel  Qun L Thuc/div_Tt c danh mcCha phn loiThuc gim au'), 
    0)

WebUI.closeBrowser()

