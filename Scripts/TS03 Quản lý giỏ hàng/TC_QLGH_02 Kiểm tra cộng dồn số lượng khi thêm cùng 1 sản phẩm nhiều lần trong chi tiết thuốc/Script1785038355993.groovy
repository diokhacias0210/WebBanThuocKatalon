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

WebUI.setText(findTestObject('Page_PharmaCare/input_Nhp s in thoi'), '0913420982')

WebUI.setEncryptedText(findTestObject('Page_PharmaCare/input_Nhp mt khu'), 'aeHFOx8jV/A=')

WebUI.click(findTestObject('Page_PharmaCare/button_ng nhp'))

WebUI.click(findTestObject('Page_PharmaCare  Nh thuc trc tuyn/i_fa-solid fa-bars'))

WebUI.click(findTestObject('Page_PharmaCare  Nh thuc trc tuyn/a_Danh sch hng ha'))

WebUI.click(findTestObject('Page_Danh sch sn phm thuc  PharmaCare/img_Vin si Tovalgan Ef Trng Th Pharma'))

WebUI.click(findTestObject('Page_Vin si Tovalgan Ef Trng Th Pharma  PharmaCare/i_fa-solid fa-plus'))

WebUI.click(findTestObject('Page_Vin si Tovalgan Ef Trng Th Pharma  PharmaCare/i_fa-solid fa-minus'))

WebUI.click(findTestObject('Page_Vin si Tovalgan Ef Trng Th Pharma  PharmaCare/button_Thm vo gi hng'))

WebUI.rightClick(findTestObject('Page_Vin si Tovalgan Ef Trng Th Pharma  PharmaCare/a_Gi hng'))

WebUI.assertElementPresent(findTestObject('Page_Vin si Tovalgan Ef Trng Th Pharma  PharmaCare/a_Gi hng'), 0)

WebUI.closeBrowser()

