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

WebUI.navigateToUrl(GlobalVariable.URL_11)

WebUI.maximizeWindow()

WebUI.callTestCase(findTestCase('Login/Login other 510'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('CostMaintainPage/costMenu'))

WebUI.click(findTestObject('CostMaintainPage/costMaintainPage'))

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

WebUI.click(findTestObject('CostMaintainPage/itemTypeDropdownIcon'))

'Unable to select Item Type value on firefox'
WebUI.click(findTestObject('CostMaintainPage/itemType_CLINK'))

WebUI.setText(findTestObject('CostMaintainPage/input_Costlink'), input_CostLinkID)

WebUI.sendKeys(findTestObject('CostMaintainPage/input_Costlink'), Keys.chord(Keys.ENTER))

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

vend_nbr_ui = WebUI.getAttribute(findTestObject('CostMaintainPage/vendor_nbr'), 'value')

create_by_ui = WebUI.getText(findTestObject('CostMaintainPage/search_createBy'))

WebUI.closeBrowser()

