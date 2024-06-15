import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { H1, H3, H4} from "@/components/ui/typography";
import { Link } from "react-router-dom";

export default function HomePage() {
    return (
        <div>
            <div className="text-center">
                <H1 className="p-6">AeroNode</H1>
                <div className="py-12">
                    <H3>
                        adding autonomy to drones shouldnt be hard.
                    </H3>
                    <br />
                    <H3>
                        aeronode makes it easy.
                    </H3>
                </div>
            </div>
            <div className="flex flex-row p-6 space-x-4 text-justify">
                <Card className="w-1/2 flex flex-col">
                    <CardHeader>
                        <CardTitle>
                            Docs
                        </CardTitle>
                    </CardHeader>
                    <CardContent className="grow">
                        <H4>
                        Check out the docs to get started with aeronode.
                        </H4>
                    </CardContent>
                    <CardFooter>
                        <Link to={"https://docs.aeronode.io"}>
                            <Button>
                                Get Started
                            </Button>
                        </Link>
                    </CardFooter>
                </Card>
                <Card className="w-1/2">
                    <CardHeader>
                        <CardTitle>
                            Code
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <H4>
                        Aeronode is open source.

                        </H4>
                        <br />
                        <H4>
                        All of it: the board, the firmware, and the api.

                        </H4>
                        <br />
                        Check it out on github.
                    </CardContent>
                    <CardFooter>
                        <Link to={"https://github.com/aeronode"}>
                            <Button>
                                Github
                            </Button>
                        </Link>
                    </CardFooter>
                </Card>
            </div>
        </div>
    );
}