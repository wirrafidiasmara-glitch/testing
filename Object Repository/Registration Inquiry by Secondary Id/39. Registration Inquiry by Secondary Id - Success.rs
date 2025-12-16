<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>39. Registration Inquiry by Secondary Id - Success</name>
   <tag></tag>
   <elementGuidId>230b07b0-98a5-4b8d-ab04-e022f9efb902</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;proxyRegInquiryRequest\&quot;: {\n\t\t\&quot;transactionId\&quot;: \&quot;${transactionId}\&quot;,\n\t\t\&quot;transactionCode\&quot;: \&quot;${transactionCode}\&quot;,\n\t\t\&quot;channelType\&quot;: \&quot;${channelType}\&quot;,\n\t\t\&quot;institutionBic\&quot;: \&quot;${institutionBic}\&quot;,\n\t\t\&quot;customerSecondaryType\&quot;: \&quot;${customerSecondaryType}\&quot;,\n\t\t\&quot;registrationId\&quot;: \&quot;${registrationId}\&quot;,\n\t\t\&quot;customerAccountNumber\&quot;: \&quot;${customerAccountNumber}\&quot;,\n\t\t\&quot;customerSecondaryValue\&quot;: \&quot;${customerSecondaryValue}\&quot;\n\t}\n}&quot;,
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
      <name>X-IP-ADDRESS</name>
      <type>Main</type>
      <value>${X_IP_ADDRESS}</value>
      <webElementGuid>300f736c-a587-406f-8498-89725e1bf564</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-DEVICE-ID</name>
      <type>Main</type>
      <value>${X_DEVICE_ID}</value>
      <webElementGuid>09c81924-cc73-471c-9b96-cac6804a7e56</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-LATITUDE</name>
      <type>Main</type>
      <value>${X_LATITUDE}</value>
      <webElementGuid>219a9c08-851c-4168-97f9-5c5c0f55e81e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-LONGITUDE</name>
      <type>Main</type>
      <value>${X_LONGITUDE}</value>
      <webElementGuid>54e842b8-a5ca-4222-9aaa-9ddaef089238</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QWRtaW5pc3RyYXRvcjptYW5hZ2U=</value>
      <webElementGuid>2de04c8f-ed7b-4536-ace8-71b44630457f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${path}/invoke/KomiBifastOriginProxy.interfaces:proxyRegInqBySecondaryId</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.IS_Postgres_Origin</defaultValue>
      <description></description>
      <id>cc821565-57e2-4274-80b9-5d28aa1d5f08</id>
      <masked>false</masked>
      <name>path</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e84182b6-fe0a-40cc-bc8f-6b6fa37e158d</id>
      <masked>false</masked>
      <name>X_IP_ADDRESS</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a3081832-a3c8-4865-b8e4-c704b345dcb8</id>
      <masked>false</masked>
      <name>X_DEVICE_ID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>151902ad-8bb7-4eb7-9744-a3684458396d</id>
      <masked>false</masked>
      <name>X_LATITUDE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0406c368-1365-4a28-88b7-3fb54d0c5964</id>
      <masked>false</masked>
      <name>X_LONGITUDE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5be3c7e0-5b5f-474f-8f55-3a01734a4488</id>
      <masked>false</masked>
      <name>transactionId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>75a51dca-af0f-4d0d-ab05-86b9335c0c56</id>
      <masked>false</masked>
      <name>transactionCode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b01b7c2d-7d7f-48aa-8ec8-6bc49e602b81</id>
      <masked>false</masked>
      <name>channelType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9e266824-ecee-4b6a-b039-2b1b5442b5b9</id>
      <masked>false</masked>
      <name>institutionBic</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6ba1e518-366c-4818-aa1b-450b282623f9</id>
      <masked>false</masked>
      <name>customerSecondaryType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6b2fac18-5ae4-4c77-a28f-96240bc5eaf9</id>
      <masked>false</masked>
      <name>registrationId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3f480aa6-4c1b-4b49-9dcd-4219e1bab3d1</id>
      <masked>false</masked>
      <name>customerAccountNumber</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>94dd6857-7108-4980-ab6f-844eba0cb737</id>
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
