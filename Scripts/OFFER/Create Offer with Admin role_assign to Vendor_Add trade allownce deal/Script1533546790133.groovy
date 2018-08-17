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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://w2capl0051511.heb.com:20143/DCM_UI/login')

WebUI.maximizeWindow()

WebUI.callTestCase(findTestCase('Login/Login other 510'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Page_Home - Cost and Deals/a_Offer'))

WebUI.click(findTestObject('Page_Create Offer - Cost and Deals/open_offer_create'))

WebUI.setText(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/input_vendorId'), '17349')

WebUI.click(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/a_17349 - KEHE FOOD DISTR INC'))

WebUI.setText(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/input_vendorId'), '17349 - KEHE FOOD DISTR INC - DSD')

WebUI.setText(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/input_descriptionId'), 'test')

WebUI.setText(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/textarea_commentId'), 'comment')

WebUI.setText(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/textarea_HEBCommentId'), 'HEb comment')

WebUI.setText(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/input_assignedToIdVendor'), 'v')

WebUI.click(findTestObject('Page_Create Offer - Cost and Deals/a_v903579 - Ryan Lawrence(Brok'))

WebUI.click(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/span_nextOffer'))

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

newOfferID = WebUI.getAttribute(findTestObject('Page_Create Offer - Cost and Deals/new_offer_id'), 'value')

WebUI.closeBrowser()

