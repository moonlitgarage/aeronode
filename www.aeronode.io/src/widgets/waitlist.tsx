import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Link } from "react-router-dom";

export default function WaitlistWidget() {
    return (
        <div>
            <Card>
                <CardHeader>
                <CardTitle>
                    Waitlist
                </CardTitle>
                </CardHeader>
                
                <CardContent>
                Join the waitlist for updates and to be the first to know when we launch!
                </CardContent>
                <CardFooter>
                <Link to="https://forms.office.com/r/0MevCAKSCH"><Button>Join Here!</Button></Link>
                </CardFooter>
            </Card>
        </div>
    );
}
