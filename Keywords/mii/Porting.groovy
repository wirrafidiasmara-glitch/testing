package mii
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords

import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException
import groovy.json.JsonSlurper
import com.kms.katalon.core.testdata.InternalData

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import com.google.gson.Gson
import com.google.gson.GsonBuilder
import com.google.gson.JsonParser as JParser
import com.google.gson.JsonElement


public class Porting {

	@Keyword
	def compareResponseCode(def bodyResponse, String message){
		println(bodyResponse)
		def parser = new JsonSlurper().parseText(bodyResponse)
		println('parser' +parser)

		def result = parser['receivingProxyNotifyPortingResponse']['responseCode']

		println("responseCode" + result)

		if(result == message) {
			KeywordUtil.markPassed("Actual Response Code Match Test Data Response Code")
		} else {
			KeywordUtil.markFailed("Expected Response Code Not Match Actual Response Code. Expected Response Code: " +message+ " and Actual Response Code: "+result)
		}
	}


	@Keyword
	def compareResponseMessage(def bodyResponse, String message){
		println(bodyResponse)
		def parser = new JsonSlurper().parseText(bodyResponse)
		println('parser' +parser)

		def result = parser['receivingProxyNotifyPortingResponse']['responseMessage']

		println("responseMessage" + result)

		if(result == message) {
			KeywordUtil.markPassed("Actual Response Message Match Test Data Response Message")
		} else {
			KeywordUtil.markFailed("Expected Response Message Not Match Actual Response Message. Expected Response Message: " +message+ " and Actual Response Message: "+result)
		}
	}
}