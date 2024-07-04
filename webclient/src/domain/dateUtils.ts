export function shortenDurationStr(date: string) {
  if (date.includes("seconds")) {
    return date.replace("seconds", "sec");
  } else if (date.includes("minute")) {
    return date.replace("minute", "min");
  } else if (date.includes("hours")) {
    return date.replace("hours", "hr");
  } else if (date.includes("days")) {
    return date.replace("days", "d");
  } else if (date.includes("weeks")) {
    return date.replace("weeks", "wk");
  } else if (date.includes("months")) {
    return date.replace("months", "m");
  } else if (date.includes("years")) {
    return date.replace("years", "y");
  }
  return "";
}
