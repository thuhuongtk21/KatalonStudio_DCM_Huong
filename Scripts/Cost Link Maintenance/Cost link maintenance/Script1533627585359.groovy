import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.openqa.selenium.By as By
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.navigateToUrl('https://w2capl0051511.heb.com:20143/DCM_UI/login')

WebUI.maximizeWindow()

WebUI.callTestCase(findTestCase('Login/Login other 510'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Cost Link Maintenance Page/item_menu'))

WebUI.click(findTestObject('Cost Link Maintenance Page/costlink_maintenance_menu'))

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

WebUI.setText(findTestObject('Cost Link Maintenance Page/costlink_id'), '6')

WebUI.click(findTestObject('Cost Link Maintenance Page/search_button'))

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

List<String> item_code_list_ui = CustomKeywords.'getListValue.fromDatatable.getGetListValueOnDatatable'('//label[@class=\'item-code\']')

String itm_id_db = null

List<String> item_code_list_db = new ArrayList()

def resultSet = CustomKeywords.'connectToDB.connectToDB2Test.executeQuery'('select distinct ITM_ID from db2tst6.VEND_LOC_ITM where CST_LNK_ID = 6')

while (resultSet.next()) {
    itm_id_db = resultSet.getObject('ITM_ID')

    item_code_list_db.add(itm_id_db)
}

WebUI.verifyEqual(item_code_list_ui, item_code_list_db)


WebUI.closeBrowser()

