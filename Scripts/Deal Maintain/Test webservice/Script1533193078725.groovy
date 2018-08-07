import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import javax.wsdl.xml.WSDLLocator

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.sun.xml.internal.ws.api.WSBinding



def response = WS.sendRequest(findTestObject('WebService/ActiveDealService_Http'))

WS.verifyElementPropertyValue(response, 'GetEntireActiveDeal_Reply.ActiveDealHeader[0].DEAL_TYPE_CODE', '')




WS.verifyElementText(response, "GetEntireActiveDeal_Reply.ActiveDealHeader[0].DEAL_TYPE_CODE".trim(), 'TPR')

