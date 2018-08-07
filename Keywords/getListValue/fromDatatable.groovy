package getListValue

import org.openqa.selenium.By as By
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.webui.driver.DriverFactory

public class fromDatatable {

	@Keyword
	def public getGetListValueOnDatatable(String locator) {
		WebDriver driver = DriverFactory.getWebDriver()

		List<String> elementList = driver.findElements(By.xpath(locator))

		List<String> value_list_ui = new ArrayList()

		for (WebElement element : elementList) {
			String value = element.getText()

			value_list_ui.add(value)
		}
		return value_list_ui;
	}
}
