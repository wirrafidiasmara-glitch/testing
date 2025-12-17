<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>4. System Notification - Invalid Mandatory Field</name>
   <tag></tag>
   <elementGuidId>f6f27a5b-7213-4374-be37-92c29e2cf19b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;receivingSystemNotificationMessageRequest\&quot;: {\n\t\t\&quot;eventCode\&quot;: \&quot;${eventCode}\&quot;,\n\t\t\&quot;eventParams1\&quot;: \&quot;${eventParams}\&quot;,\n\t\t\&quot;eventDescription\&quot;: \&quot;${eventDescription}\&quot;,\n\t\t\&quot;eventDateTime\&quot;: \&quot;${eventDateTime}\&quot;\n\t}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>991fec3b-bfa6-4517-ace2-60019ce9dfe8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dXNlcmNoYW5uZWw6dXNlcmNoYW5uZWwxMjM=</value>
      <webElementGuid>a1ea184b-8a17-4935-8aef-bb74f1d3c768</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${path}/invoke/KomiBifastReceivingSystem.interfaces:systemNotificationMessage</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.IS_ORACLE_Incoming</defaultValue>
      <description></description>
      <id>cc821565-57e2-4274-80b9-5d28aa1d5f08</id>
      <masked>false</masked>
      <name>path</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d7765d3f-8a7d-4d73-8d27-e9e5c155eedc</id>
      <masked>false</masked>
      <name>eventCode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>cb509959-6226-45d1-b5a0-72238333d86a</id>
      <masked>false</masked>
      <name>eventParams</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c43b6a6f-6ea1-41e3-90db-c0dae9ae3b10</id>
      <masked>false</masked>
      <name>eventDescription</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2a00e890-c235-42a5-9e8b-566e845be729</id>
      <masked>false</masked>
      <name>eventDateTime</name>
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
