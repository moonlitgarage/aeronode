// networkUtils.ts
import { ENDPOINT_LOOPS } from "@/common/constants";

export async function submitWaitlistForm(name: string, email: string): Promise<Response> {
  const userGroup = "0";
  const formBody = `firstName=${encodeURIComponent(name)}&email=${encodeURIComponent(email)}&userGroup=${encodeURIComponent(userGroup)}`;
  
  const response = await fetch(ENDPOINT_LOOPS, {
    method: "POST",
    body: formBody,
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
  });

  if (!response.ok) {
    throw new Error('Failed to submit the form');
  }

  return response;
}