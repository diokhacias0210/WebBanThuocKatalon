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

WebUI.click(findTestObject('TC_QLTVDM_29/Page_PharmaCare  Nh thuc trc tuyn/a_Xem tt c'))

WebUI.click(findTestObject('TC_QLTVDM_29/Page_Danh sch sn phm thuc  PharmaCare/input_Khng k n (OTC)'))

WebUI.rightClick(findTestObject('TC_QLTVDM_29/Page_Danh sch sn phm thuc  PharmaCare/img_Bt Glotadol 250 Abbott'))

WebUI.assertElementPresent(findTestObject('TC_QLTVDM_29/Page_Danh sch sn phm thuc  PharmaCare/img_Bt Glotadol 250 Abbott'), 
    0)

WebUI.closeBrowser()

