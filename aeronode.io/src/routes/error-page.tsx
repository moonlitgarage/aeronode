import { Button } from "@/components/ui/button";
import { Frown } from "lucide-react";
import { Link, useRouteError } from "react-router-dom";

interface ErrorWithStatus extends Error {
    statusText?: string;
    status?: number;
}

/**
 * v0 by Vercel.
 * @see https://v0.dev/t/O3be2k6pWyB
 * Documentation: https://v0.dev/docs#integrating-generated-code-into-your-nextjs-app
 */
export default function ErrorPage() {
    const error = useRouteError() as ErrorWithStatus;
    console.error(error);

    return (
        <div className="flex h-[100dvh] flex-col items-center justify-center bg-white px-4 py-16 dark:bg-gray-950">
            <div className="mx-auto flex max-w-md flex-col items-center justify-center space-y-6">
                <div className="text-9xl font-bold text-gray-900 dark:text-gray-50"><Frown className="h-10 w-10" /></div>
                <div className="text-9xl font-bold text-gray-900 dark:text-gray-50">{error.status}</div>
                <div className="space-y-1 text-center">
                    <h1 className="text-3xl font-bold tracking-tight text-gray-900 dark:text-gray-50 sm:text-4xl">
                        Oops! {error.statusText}.
                    </h1>
                    <div className="p-8">
                        <Link to={`/`}>
                            <Button variant="outline">Go back home</Button>
                        </Link>
                    </div>
                </div>
            </div>
        </div>
    )
}
