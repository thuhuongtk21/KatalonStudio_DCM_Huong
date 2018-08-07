import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://w2capl0051511.heb.com:20143/DCM_UI/login')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Login Page/input_j_username'), 'p165114')

WebUI.setEncryptedText(findTestObject('Login Page/input_j_password'), '9KzOjDimb7zsqjjOXXaQnQ==')

WebUI.click(findTestObject('Login Page/button_Login'))

WebUI.click(findTestObject('Home Page/a_Deals'))

WebUI.click(findTestObject('Home Page/a_Maintain'))

WebUI.setText(findTestObject('Deal Maintenance Page/input_deal-search'), findTestData('Deal Maintain/DealID').getValue(1, 
        1))

WebUI.click(findTestObject('Deal Maintenance Page/span_deal-search-btn'))

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

String dealDescription = WebUI.getAttribute(findTestObject('Deal Maintenance Page/input_description'), 'value')

response = WS.sendRequest(findTestObject('WebService/ActiveDealService_Http'))

String dealDescription_WS = response.getResponseText('DEAL_DES')

WS.verifyEqual(dealDescription, 'response.DEAL_DES')

WebUI.closeBrowser()

