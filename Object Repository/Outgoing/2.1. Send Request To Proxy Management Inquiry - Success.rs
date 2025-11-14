<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>2.1. Send Request To Proxy Management Inquiry - Success</name>
   <tag></tag>
   <elementGuidId>8999525f-f79c-4623-86d8-a4f959a9e1df</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;proxyManagementInquiryRequest\&quot;: {\n        \&quot;transactionId\&quot;: \&quot;${transactionId}\&quot;,\n        \&quot;institutionBic\&quot;: \&quot;${institutionBic}\&quot;,\n        \&quot;customerAccountNumber\&quot;: \&quot;${customerAccountNumber}\&quot;,\n        \&quot;proxyOperationType\&quot;: \&quot;${proxyOperationType}\&quot;,\n        \&quot;proxyType\&quot;: \&quot;${proxyType}\&quot;,\n        \&quot;proxyAlias\&quot;: \&quot;${proxyAlias}\&quot;,\n        \&quot;channelType\&quot;: \&quot;${channelType}\&quot;,\n        \&quot;customerSecondaryType\&quot;: \&quot;${customerSecondaryType}\&quot;,\n        \&quot;customerSecondaryValue\&quot;: \&quot;${customerSecondaryValue}\&quot;       \n        \n    }\n}\n\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>${Content_Type}</value>
      <webElementGuid>991fec3b-bfa6-4517-ace2-60019ce9dfe8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dXNlcmNoYW5uZWw6dXNlcmNoYW5uZWwxMjM=</value>
      <webElementGuid>84804065-760f-4e48-ad09-d94d8859fc4a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://184.169.41.163:5555/invoke/KomiBifastOriginProxy.interfaces:proxyInquiry</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9a396a94-a406-43ad-ad93-7282345811a6</id>
      <masked>false</masked>
      <name>institutionBic</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>cde7127b-a863-4d87-932b-6ececb1843b7</id>
      <masked>false</masked>
      <name>Content_Type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7f58420a-9bc0-4b25-ba22-c57dc105faf5</id>
      <masked>false</masked>
      <name>transactionId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f78d9eaf-63bb-4147-97b1-b470879e6c19</id>
      <masked>false</masked>
      <name>customerAccountNumber</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>def59953-06a0-45de-bfbe-9cb1b2b2ab4c</id>
      <masked>false</masked>
      <name>proxyOperationType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>407c775e-0497-4b2d-ad95-d9c55c22dd6c</id>
      <masked>false</masked>
      <name>proxyType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5babdce3-fac8-457c-bf82-24bc0f14c977</id>
      <masked>false</masked>
      <name>proxyAlias</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>20325a70-a76c-4525-84eb-fbdfa71eb618</id>
      <masked>false</masked>
      <name>channelType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>857ffc9f-277b-43ee-ae45-515e4489b045</id>
      <masked>false</masked>
      <name>customerSecondaryType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f33bd39d-1a2e-447b-af10-a5305316f517</id>
      <masked>false</masked>
      <name>customerSecondaryValue</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
