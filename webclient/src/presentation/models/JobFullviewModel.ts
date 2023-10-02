import { getCompanies } from "../../domain/repository/CompanyRepo";
import { getCountries } from "../../domain/repository/CountryRepo";
import { getIndustries } from "../../domain/repository/IndustryRepo";
import { getLanguages } from "../../domain/repository/LanguageRepo";
import { getSalaries } from "../../domain/repository/SalaryRepo";
import { appendPlusLargeCurrency } from "../common/CurrencyFormatter";
import { OptionType } from "../components/controls/DropDown";

export interface JobPostOptions {
  companies: OptionType[];
  countries: OptionType[];
  languages: OptionType[];
  industries: OptionType[];
  salaries: OptionType[];
}

export async function getJobPostOptions(): Promise<JobPostOptions> {
  const [companies, countries, languages, industries, salaries] =
    await Promise.all([
      getCompanies(),
      getCountries(),
      getLanguages(),
      getIndustries(),
      getSalaries(),
    ]);

  return {
    companies: companies.map((company) => {
      return { name: company.name, value: company.id };
    }),
    countries: countries.map((country) => {
      return { name: country.name, value: country.id };
    }),
    languages: languages.map((language) => {
      return { name: language.name, value: language.id };
    }),
    industries: industries.map((industry) => {
      return { name: industry.name, value: industry.id };
    }),
    salaries: salaries.map((salary) => {
      return {
        name: appendPlusLargeCurrency(salary.base),
        value: salary.id,
      };
    }),
  };
}
