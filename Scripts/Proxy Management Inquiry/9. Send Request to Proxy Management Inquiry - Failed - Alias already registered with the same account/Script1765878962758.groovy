import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import mii.JsonParser as JsonParser
import org.apache.commons.lang.RandomStringUtils as RandStr
import java.util.Random as Random

def variable = [:]

variable.put('transactionId', transactionId)

variable.put('institutionBic', institutionBic)

variable.put('customerAccountNumber', customerAccountNumber)

variable.put('proxyOperationType', proxyOperationType)

variable.put('proxyType', proxyType)

variable.put('proxyAlias', proxyAlias)

variable.put('channelType', channelType)

variable.put('customerSecondaryType', customerSecondaryType)

variable.put('customerSecondaryValue', customerSecondaryValue)

variable.put('X_IP_ADDRESS', X_IP_ADDRESS)

variable.put('X_DEVICE_ID', X_DEVICE_ID)

variable.put('X_LATITUDE', X_LATITUDE)

variable.put('X_LONGITUDE', X_LONGITUDE)

RequestObject request = findTestObject('Proxy Management Inquiry/5. Send Request to Proxy Management Inquiry - Success', 
    variable)

def response = WS.sendRequest(request)

def bodyResponse = response.getResponseBodyContent()

WS.comment(bodyResponse)

JsonParser.prettier(bodyResponse)

CustomKeywords.'mii.ProxyManagementInquiry.compareResponseCode'(bodyResponse, '0000')

