import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";

import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import { ENDPOINT_LOOPS } from "@/common/constants";

const formSchema = z.object({
  name: z.string().min(1, { message: "Name must be at least 1 character." }),
  email: z.string().email({ message: "Please enter a valid email address." }),
})

export function WaitlistForm() {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
      email: "",
    },
  })

  function handleSubmit(name: string, email: string) {
    const userGroup = "2";
    const formBody = `firstName=${encodeURIComponent(name)}&email=${encodeURIComponent(email)}&userGroup=${encodeURIComponent(userGroup)}`;  
    fetch(ENDPOINT_LOOPS, {
      method: "POST",
      body: formBody,
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
      },
    });
  }
  

  function onSubmit(values: z.infer<typeof formSchema>) {
    handleSubmit(values.name, values.email);
  }

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
        <FormField
          control={form.control}
          name="name"
          render={({ field }) => (
            <FormItem>
              <FormControl>
                <Input placeholder="Your Name" {...field}/>
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="email"
          render={({ field }) => (
            <FormItem>
              <FormControl>
                <Input placeholder="Your Email" {...field}/>
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit" className="w-full font-semibold py-2 px-4">
          Join Waitlist
        </Button>
      </form>
    </Form>
  )
}

export default function WaitlistWidget() {
  return (
    <div className="flex justify-center items-center">
      <Card className="w-full max-w-md">
        <CardHeader className="text-center">
          <CardTitle className="text-2xl font-bold">
            Waitlist
          </CardTitle>
        </CardHeader>
        <CardContent>
          <p className="mb-6 text-center">
          Join the waitlist for updates and to be the first to know when we launch!          
          </p>
          <WaitlistForm />
        </CardContent>
      </Card>
    </div>
  );
}