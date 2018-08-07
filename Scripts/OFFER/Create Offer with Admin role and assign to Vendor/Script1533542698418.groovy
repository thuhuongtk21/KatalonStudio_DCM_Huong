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

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

WebUI.setText(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/input_descriptionId'), 'test')

WebUI.setText(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/textarea_commentId'), 'comment')

WebUI.setText(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/textarea_HEBCommentId'), 'HEb comment')

WebUI.setText(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/input_assignedToIdVendor'), 'v')

WebUI.click(findTestObject('Page_Create Offer - Cost and Deals/a_v903579 - Ryan Lawrence(Brok'))

WebUI.setText(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/input_keyword'), 'keyword')

WebUI.click(findTestObject('Object Repository/Page_Create Offer - Cost and Deals/span_nextOffer'))

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

newOfferID = WebUI.getAttribute(findTestObject('Page_Create Offer - Cost and Deals/new_offer_id'), 'value')

WebUI.click(findTestObject('Deal Builder Page/deal_type_button'))

WebUI.click(findTestObject('Deal Builder Page/trade_allowance'))

WebUI.click(findTestObject('Deal Builder Page/purchase_deal'))

WebUI.click(findTestObject('Deal Builder Page/item_level'))

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

WebUI.doubleClick(findTestObject('Deal Builder Page/prod_id'))

WebUI.waitForElementNotPresent(findTestObject('Deal Builder Page/add_item_to_deal_Loading'), 60)

WebUI.click(findTestObject('Deal Builder Page/location_group_button'))

WebUI.waitForElementNotPresent(findTestObject('Deal Builder Page/product_location_loading'), 60)

WebUI.doubleClick(findTestObject('Deal Builder Page/cos_group_value'))

WebUI.waitForElementNotPresent(findTestObject('Deal Builder Page/add_item_to_deal_Loading'), 60)

WebUI.executeJavaScript('window.scrollTo(0, document.body.scrollHeight)', [])

WebUI.click(findTestObject('Deal Builder Page/click_start'))

WebUI.setText(findTestObject('Deal Builder Page/input_start_date'), '8/3/2018')

WebUI.click(findTestObject('Deal Builder Page/click_end_column'))

WebUI.setText(findTestObject('Deal Builder Page/input_end_date'), '8/30/2018')

WebUI.closeBrowser()

