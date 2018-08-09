import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

def response = WS.sendRequest(findTestObject('WebService/ActiveDealService_Http'))

String dealTypeCode = response.getProperty("GetEntireActiveDeal_Reply.ActiveDealHeader[0].DEAL_TYPE_CODE") 

WS.verifyElementText(response, 'GetEntireActiveDeal_Reply.ActiveDealHeader[0].DEAL_TYPE_CODE'.trim(), 'TPR')


