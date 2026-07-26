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

WebUI.setText(findTestObject('New Folder (1)/Page_PharmaCare/input_Nhp s in thoi'), '0911234567')

WebUI.setEncryptedText(findTestObject('New Folder (1)/Page_PharmaCare/input_Nhp mt khu'), 'aeHFOx8jV/A=')

WebUI.click(findTestObject('New Folder (1)/Page_PharmaCare/button_ng nhp'))

WebUI.click(findTestObject('New Folder (1)/Page_PharmaCare  PharmaCare  H s dc s/a_Duyt thuc k n'))

WebUI.setText(findTestObject('New Folder (1)/Page_PharmaCare  Duyt n thuc/input_Tm theo m yu cu hoc tn khch hng'), 'lê văn c')

WebUI.rightClick(findTestObject('New Folder (1)/Page_PharmaCare  Duyt n thuc/div_L Vn C'))

WebUI.assertElementPresent(findTestObject('New Folder (1)/Page_PharmaCare  Duyt n thuc/div_L Vn C'), 0)

