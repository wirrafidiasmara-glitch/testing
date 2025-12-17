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

variable.put('requestID', requestID)

variable.put('requestDate', requestDate)

variable.put('recipientBIC', recipientBIC)

variable.put('originalID', originalID)

variable.put('proxyType', proxyType)

variable.put('proxyValue', proxyValue)

variable.put('originalAccountregistrationID', originalAccountregistrationID)

variable.put('originalAccountdisplayName', originalAccountdisplayName)

variable.put('originalAccountbankID', originalAccountbankID)

variable.put('originalAccountaccountIdentifier', originalAccountaccountIdentifier)

variable.put('originalAccountaccountType', originalAccountaccountType)

variable.put('originalAccountaccountName', originalAccountaccountName)

variable.put('originalAccountcustomerType', originalAccountcustomerType)

variable.put('originalAccountcustomerID', originalAccountcustomerID)

variable.put('originalAccountcustomerRsdntSts', originalAccountcustomerRsdntSts)

variable.put('originalAccountcustomerTwnNm', originalAccountcustomerTwnNm)

variable.put('newAccountregistrationID', newAccountregistrationID)

variable.put('newAccountdisplayName', newAccountdisplayName)

variable.put('newAccountbankID', newAccountbankID)

variable.put('newAccountaccountIdentifier', newAccountaccountIdentifier)

variable.put('newAccountaccountType', newAccountaccountType)

variable.put('newAccountaccountName', newAccountaccountName)

variable.put('newAccountcustomerType', newAccountcustomerType)

variable.put('newAccountcustomerID', newAccountcustomerID)

variable.put('newAccountcustomerRsdntSts', newAccountcustomerRsdntSts)

variable.put('newAccountcustomerTwnNm', newAccountcustomerTwnNm)

RequestObject request = findTestObject('Porting/1. Proxy Porting - Sukses', variable)

def response = WS.sendRequest(request)

def bodyResponse = response.getResponseBodyContent()

WS.comment(bodyResponse)

JsonParser.prettier(bodyResponse)

CustomKeywords.'mii.Porting.compareResponseCode'(bodyResponse, '20072000')

