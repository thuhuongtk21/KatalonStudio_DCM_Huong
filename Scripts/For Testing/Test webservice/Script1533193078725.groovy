import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

def response = WS.sendRequest(findTestObject('WebService/ActiveDealService_Http'))
response.getResponseText(response, 'GetEntireActiveDeal_Reply.ActiveDealHeader.DEAL_TYPE_CODE')



WS.verifyElementText(response, 'GetEntireActiveDeal_Reply.ActiveDealHeader.DEAL_TYPE_CODE', 'TPR')


