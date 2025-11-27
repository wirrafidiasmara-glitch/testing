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

def variable = [:]

variable.put('Content_Type', Content_Type)

variable.put('channelType', channelType)

variable.put('institutionBic', institutionBic)
	
variable.put('transactionId', transactionId)

variable.put('transactionCode', transactionCode)

variable.put('proxyOperationType', proxyOperationType)

variable.put('proxyType', proxyType)

variable.put('proxyAlias', proxyAlias)

variable.put('customerAccountNumber', customerAccountNumber)

variable.put('branchCode', branchCode)

variable.put('branchInput', branchInput)

variable.put('registrationId', registrationId)

variable.put('accountIdentifier', accountIdentifier)

variable.put('customerType', customerType)

variable.put('customerResidentStatus', customerResidentStatus)

variable.put('customerCityCode', customerCityCode)

variable.put('customerAccountType', customerAccountType)

variable.put('customerAccountName', customerAccountName)

variable.put('customerSecondaryType', customerSecondaryType)

variable.put('customerSecondaryValue', customerSecondaryValue)

variable.put('customerCurrency', customerCurrency)

variable.put('cifNumber', cifNumber)

variable.put('displayName', displayName)

variable.put('proxyInformation', proxyInformation)

RequestObject request = findTestObject('Outgoing/4.12. ProxyAlias Deregistration (Delete)- Failed -mandatory field tidak diisi', variable)

def response = WS.sendRequest(request)

def bodyResponse = response.getResponseBodyContent()

WS.comment(bodyResponse)

JsonParser.prettier(bodyResponse)

CustomKeywords.'mii.ProxyManagement.compareResponseCode'(bodyResponse, '0000')

