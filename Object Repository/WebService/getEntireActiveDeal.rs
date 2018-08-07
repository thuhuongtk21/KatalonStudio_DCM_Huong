<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getEntireActiveDeal</name>
   <tag></tag>
   <elementGuidId>5be17b76-0de4-4690-aa59-0f087a840f4c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:get=&quot;http://xmlns.heb.com/ei/ActiveDealService/GetEntireActiveDeal_Request&quot; xmlns:aut=&quot;http://xmlns.heb.com/ei/Authentication&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;get:GetEntireActiveDeal_Request>
         &lt;get:DEAL_ID>1003261&lt;/get:DEAL_ID>
         &lt;aut:Authentication>
            &lt;aut:USER_ID>?&lt;/aut:USER_ID>
            &lt;aut:PWD>?&lt;/aut:PWD>
            &lt;aut:CLIENT_ID>?&lt;/aut:CLIENT_ID>
         &lt;/aut:Authentication>
      &lt;/get:GetEntireActiveDeal_Request>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>getEntireActiveDeal</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')

assertThat(jsonResponse.issues[0].fields.project.key).isEqualTo('KTP')
</verificationScript>
   <wsdlAddress>https://coreapi.cert.heb.com:8443/SOAP/v1/ActiveDealService?wsdl</wsdlAddress>
</WebServiceRequestEntity>
