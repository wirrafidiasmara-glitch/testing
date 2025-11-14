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

def variable = [:]

variable.put('Content_Type', Content_Type)

variable.put('channelType', channelType)

variable.put('institutionBic', institutionBic)

variable.put('transactionId', transactionId)

variable.put('debitedAccountNumber', debitedAccountNumber)

variable.put('creditedBic', creditedBic)

variable.put('creditedProxyType', creditedProxyType)

variable.put('creditedProxyAlias', creditedProxyAlias)

variable.put('chargeType', chargeType)

variable.put('trxAmount', trxAmount)

variable.put('feeAmount', feeAmount)

variable.put('categoryPurpose', categoryPurpose)

variable.put('chargeBearerCode', chargeBearerCode)

RequestObject request = findTestObject('Outgoing/6.5. Account Enquiry - Failed - Invalid Mandatory Field', variable)

def response = WS.sendRequest(request)

def bodyResponse = response.getResponseBodyContent()

WS.comment(bodyResponse)

JsonParser.prettier(bodyResponse)

CustomKeywords.'mii.AccountInquiry.compareResponseCode'(bodyResponse, '2999')

