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

WebUI.setText(findTestObject('Login Page/input_j_username'), findTestData('Login/User_Login').getValue(1, 1))

WebUI.setText(findTestObject('Login Page/input_j_password'), findTestData('Login/User_Login').getValue(2, 1))

WebUI.click(findTestObject('Login Page/button_Login'))

WebUI.click(findTestObject('Object Repository/Page_Home - Cost and Deals/a_Location Group'))

WebUI.click(findTestObject('Object Repository/Page_Home - Cost and Deals/a_Create'))

WebUI.click(findTestObject('Object Repository/Page_Create Location Group - Cost a/img_addImage'))

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

WebUI.setText(findTestObject('Object Repository/Page_Create Location Group - Cost a/input_basketName'), 'add new')

WebUI.setText(findTestObject('Object Repository/Page_Create Location Group - Cost a/input_abb'), 'sssss')

WebUI.setText(findTestObject('Object Repository/Page_Create Location Group - Cost a/textarea_publicCmt'), 'cccccccccccccccc')

WebUI.setText(findTestObject('Object Repository/Page_Create Location Group - Cost a/textarea_privateCmt'), 'ssssssssssssssssssssss')

WebUI.executeJavaScript('window.scrollTo(0, document.body.scrollHeight)', [])

WebUI.click(findTestObject('Object Repository/Page_Create Location Group - Cost a/span_00006 - STEPHENVILLE'))

WebUI.click(findTestObject('Object Repository/Page_Create Location Group - Cost a/button_arrow-move-right'))

WebUI.click(findTestObject('Object Repository/Page_Create Location Group - Cost a/span_00013 - RIO GRANDE CITY-H'))

WebUI.click(findTestObject('Object Repository/Page_Create Location Group - Cost a/button_arrow-move-right'))

WebUI.executeJavaScript('document.documentElement.scrollTop = 0', [])

WebUI.click(findTestObject('Object Repository/Page_Create Location Group - Cost a/img_saveImage'))

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

WebUI.verifyElementPresent(findTestObject('Page_Create Location Group - Cost a/successed message'), 60)

String basketId = WebUI.getAttribute(findTestObject('Page_Create Location Group - Cost a/input_basketID'), 'value')

println(basketId)

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

groupName = WebUI.getAttribute(findTestObject('Page_Create Location Group - Cost a/input_basketName'), 'value')

WebUI.clearText(findTestObject('Page_Create Location Group - Cost a/input_basketName'))

WebUI.setText(findTestObject('Page_Create Location Group - Cost a/input_basketName'), 'update Basket name')

WebUI.click(findTestObject('Page_Create Location Group - Cost a/button_reset'))

WebUI.click(findTestObject('Page_Create Location Group - Cost a/confirm_button_Yes'))

WebUI.waitForElementNotPresent(findTestObject('Common/loading_bar'), 60)

groupName_reset = WebUI.getAttribute(findTestObject('Page_Create Location Group - Cost a/input_basketName'), 'value')

WebUI.verifyEqual(groupName, groupName_reset)

WebUI.closeBrowser()

