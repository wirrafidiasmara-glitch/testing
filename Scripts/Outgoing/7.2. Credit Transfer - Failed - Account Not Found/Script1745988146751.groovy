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


variable.put('Content_Type', Content_Type)

variable.put('transactionId', transactionId)

variable.put('institutionBic', institutionBic)

variable.put('channelType', channelType)

variable.put('branchCode', branchCode)

variable.put('branchInput', branchInput)

variable.put('branchFee', branchFee)

variable.put('debitedAccountName', debitedAccountName)

variable.put('debitedAccountNumber', debitedAccountNumber)

variable.put('debitedAccountType', debitedAccountType)

variable.put('debitedCityCode', debitedCityCode)

variable.put('debitedId', debitedId)

variable.put('debitedResidentStatus', debitedResidentStatus)

variable.put('debitedType', debitedType)

variable.put('creditedAccountName', creditedAccountName)

variable.put('creditedAccountNumber', creditedAccountNumber)

variable.put('creditedAccountType', creditedAccountType)

variable.put('creditedBic', creditedBic)

variable.put('creditedCityCode', creditedCityCode)

variable.put('creditedId', creditedId)

variable.put('creditedProxyAlias', creditedProxyAlias)

variable.put('creditedProxyType', creditedProxyType)

variable.put('creditedResidentStatus', creditedResidentStatus)

variable.put('creditedType', creditedType)

variable.put('categoryPurpose', categoryPurpose)

variable.put('chargeType', chargeType)

variable.put('trxAmount', trxAmount)

variable.put('feeAmount', feeAmount)

variable.put('amountCurrency', amountCurrency)

variable.put('chargeBearerCode', chargeBearerCode)

variable.put('paymentInformation', paymentInformation)

RequestObject request = findTestObject('Outgoing/7.2. Credit Transfer - Failed - Account Not Found', 
    variable)

def response = WS.sendRequest(request)

def bodyResponse = response.getResponseBodyContent()

WS.comment(bodyResponse)

JsonParser.prettier(bodyResponse)

CustomKeywords.'mii.CreditTransfer.compareResponseCode'(bodyResponse, '3603')

