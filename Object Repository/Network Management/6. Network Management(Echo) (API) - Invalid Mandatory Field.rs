<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>6. Network Management(Echo) (API) - Invalid Mandatory Field</name>
   <tag></tag>
   <elementGuidId>ce002b24-0fa8-411c-8598-875764c1604f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;receivingNetworkManagementRequest\&quot;: {\n\t\t\&quot;requestID1\&quot;: \&quot;${requestID}\&quot;,\n\t\t\&quot;requestDate\&quot;: \&quot;${requestDate}\&quot;,\n\t\t\&quot;functionCode\&quot;: \&quot;${functionCode}\&quot;,\n\t\t\&quot;transactionID\&quot;: \&quot;${transactionID}\&quot;,\n\t\t\&quot;agentID\&quot;: \&quot;${agentID}\&quot;\n\t}\n}&quot;,
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
      <webElementGuid>8b173e1d-0cb8-4735-90e6-88a3d4800b84</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${path}/invoke/KomiBifastReceivingNMS.interfaces:networkManagement</restUrl>
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
      <id>09d46258-7168-4c35-a20c-05e852b09efe</id>
      <masked>false</masked>
      <name>requestID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2349718c-2d53-47f9-9a6f-ff61689c43f1</id>
      <masked>false</masked>
      <name>requestDate</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3a01e2ee-86d4-4751-b5c9-56edcb3ffa80</id>
      <masked>false</masked>
      <name>functionCode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>754b32ce-389f-4e78-a889-226efc8115fb</id>
      <masked>false</masked>
      <name>transactionID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d3a21bd6-ac29-4404-abda-f31eb34b87b0</id>
      <masked>false</masked>
      <name>agentID</name>
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
