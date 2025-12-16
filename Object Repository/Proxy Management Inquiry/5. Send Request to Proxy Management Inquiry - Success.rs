<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>5. Send Request to Proxy Management Inquiry - Success</name>
   <tag></tag>
   <elementGuidId>a84efbcd-4f16-4e52-a8bd-45ad08596b81</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;proxyManagementInquiryRequest\&quot;: {\n\t\t\&quot;transactionId\&quot;: \&quot;${transactionId}\&quot;,\n\t\t\&quot;institutionBic\&quot;: \&quot;${institutionBic}\&quot;,\n\t\t\&quot;customerAccountNumber\&quot;: \&quot;${customerAccountNumber}\&quot;,\n\t\t\&quot;proxyOperationType\&quot;: \&quot;${proxyOperationType}\&quot;,\n\t\t\&quot;proxyType\&quot;: \&quot;${proxyType}\&quot;,\n\t\t\&quot;proxyAlias\&quot;: \&quot;${proxyAlias}\&quot;,\n\t\t\&quot;channelType\&quot;: \&quot;${channelType}\&quot;,\n\t\t\&quot;customerSecondaryType\&quot;: \&quot;${customerSecondaryType}\&quot;,\n\t\t\&quot;customerSecondaryValue\&quot;: \&quot;${customerSecondaryValue}\&quot;\n\t}\n}&quot;,
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
   <restUrl>${path}/invoke/KomiBifastOriginProxy.interfaces:proxyInquiry</restUrl>
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
      <id>d5179fa7-ba27-4b36-b8f0-374dd2d27130</id>
      <masked>false</masked>
      <name>institutionBic</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9eb8d42b-348e-4d2b-9ea5-0b549ae82842</id>
      <masked>false</masked>
      <name>customerAccountNumber</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7569b446-3750-4fc9-9ec7-2124c58c0720</id>
      <masked>false</masked>
      <name>proxyOperationType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8fe9e59a-39e2-407a-a2df-8b0d6caa9b32</id>
      <masked>false</masked>
      <name>proxyType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9873eb25-4421-401c-b1d0-109c44bc6c25</id>
      <masked>false</masked>
      <name>proxyAlias</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>56e3ad27-787a-4126-bd92-72b70ef1b11d</id>
      <masked>false</masked>
      <name>channelType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>178bc282-c97c-4a17-9d44-74d937867f48</id>
      <masked>false</masked>
      <name>customerSecondaryType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9df3b2f9-c74b-441d-b9b1-19c4e7ecfa48</id>
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
