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

WebUI.callTestCase(findTestCase('Login/Login other 510'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Page_Home - Cost and Deals/a_Offer'))

WebUI.click(findTestObject('Offer Maintain Page/offer_maintain_menu'))

WebUI.setText(findTestObject('Offer Maintain Page/offer_id_input'), '1001361')

WebUI.click(findTestObject('Offer Maintain Page/search_icon'), FailureHandling.STOP_ON_FAILURE)

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

def offer_des_ui = WebUI.getAttribute(findTestObject('Offer Maintain Page/description'), 'value')

def notify_email_id_ui = WebUI.getAttribute(findTestObject('Offer Maintain Page/notify_email_id'), 'value')

def email_id_ui = WebUI.getText(findTestObject('Offer Maintain Page/email_id'))

def resultSet = CustomKeywords.'connectToDB.connectToDB2Test.executeQuery'('select * from db2tst6.OFFER where OFFER_ID = 1001361')

String offer_des_db = null

String notify_email_id_db = null

while (resultSet.next()) {
    offer_des_db = resultSet.getObject('OFR_DES').trim()

    notify_email_id_db = resultSet.getObject('OWNER_EMAIL_ID').trim()
}

WebUI.verifyEqual(offer_des_ui, offer_des_db)

WebUI.verifyEqual(notify_email_id_ui, notify_email_id_db)

WebUI.verifyEqual(email_id_ui, notify_email_id_db)

CustomKeywords.'connectToDB.connectToDB2Test.closeDatabaseConnection'()

WebUI.closeBrowser()

