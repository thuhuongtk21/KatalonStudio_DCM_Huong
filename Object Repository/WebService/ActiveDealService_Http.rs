<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>HTTP</description>
   <name>ActiveDealService_Http</name>
   <tag></tag>
   <elementGuidId>9f327537-5150-49f0-b802-5fbc2adba13e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-Requested-With</name>
      <type>Main</type>
      <value>?db=oracle</value>
   </httpHeaderProperties>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:get=&quot;http://xmlns.heb.com/ei/ActiveDealService/GetEntireSodlDeal_Request&quot; xmlns:aut=&quot;http://xmlns.heb.com/ei/Authentication&quot; xmlns:Endpoint=&quot;http://coreapi.cert.heb.com:80/SOAP/v1/ActiveDealService?db=oracle&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;get:GetEntireSodlDeal_Request>
         &lt;get:DEAL_ID>233645&lt;/get:DEAL_ID>
         &lt;aut:Authentication>
            &lt;aut:USER_ID>?&lt;/aut:USER_ID>
            &lt;aut:PWD>?&lt;/aut:PWD>
            &lt;aut:CLIENT_ID>?&lt;/aut:CLIENT_ID>
         &lt;/aut:Authentication>
      &lt;/get:GetEntireSodlDeal_Request>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>getEntireSodlDeal</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

</verificationScript>
   <wsdlAddress>http://coreapi.cert.heb.com/SOAP/v1/ActiveDealService?wsdl</wsdlAddress>
</WebServiceRequestEntity>
