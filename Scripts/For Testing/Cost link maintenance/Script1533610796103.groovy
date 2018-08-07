import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.By as By
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement

import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

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

WebDriver driver = DriverFactory.getWebDriver()

List<String> elementList = driver.findElements(By.xpath('//label[@class=\'item-code\']'))

List<String> item_code_list_ui = new ArrayList()

for (WebElement element : elementList) {
    String value = element.getText()

    item_code_list_ui.add(value)
}

System.out.println('List value = ' + item_code_list_ui)

String itm_id_db = null
List<String> item_code_list_db = new ArrayList()

def resultSet = CustomKeywords.'connectToDB.connectToDB2Test.executeQuery'('select distinct ITM_ID from db2tst6.VEND_LOC_ITM where CST_LNK_ID = 6')

while (resultSet.next()) {
	itm_id_db = resultSet.getObject('ITM_ID')
	item_code_list_db.add(itm_id_db)
	
}

WebUI.verifyEqual(item_code_list_ui, item_code_list_db)

WebUI.closeBrowser()

