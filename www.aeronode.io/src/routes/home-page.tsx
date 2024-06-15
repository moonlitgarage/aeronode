import { LINK_DOCS, LINK_GITHUB } from "@/common/constants";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { H1, H3, List } from "@/components/ui/typography";
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
            <div className="flex flex-row p-6 space-x-4">
                <Card className="w-1/2 flex flex-col">
                    <CardHeader>
                        <CardTitle>
                            Learn how to use Aeronode
                        </CardTitle>
                    </CardHeader>
                    <CardContent className="grow">
                        <List>
                            <li>
                            Learn how to set up and use Aeronode.
                            </li>
                            <li>
                            Explore the API and firmware documentation.
                            </li>
                            <li>
                            Get started with your own drone project.
                            </li>
                        </List>
                    </CardContent>
                    <CardFooter>
                        <Link to={LINK_DOCS}>
                            <Button>
                                Read the docs
                            </Button>
                        </Link>
                    </CardFooter>
                </Card>
                <Card className="w-1/2">
                    <CardHeader>
                        <CardTitle>
                            Discover the code
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <List>
                            <li>
                            Explore Aeronode's open-source repository.
                            </li>
                            <li>
                            Access the board, firmware, and API codebase.
                            </li>
                            <li>
                            Contribute to the project or use it in your own drone applications.
                            </li>
                        </List>
                    </CardContent>
                    <CardFooter>
                        <Link to={LINK_GITHUB}>
                            <Button>
                                View on GitHub
                            </Button>
                        </Link>
                    </CardFooter>
                </Card>
            </div>
        </div>
    );
}