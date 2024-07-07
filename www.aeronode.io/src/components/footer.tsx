import { LINK_GITHUB, LINK_INSTAGRAM, LINK_TWITTER } from "@/common/constants";
import { Link } from "react-router-dom";

export default function Footer() {
    return (
        <div id="header" className="mb-0">
            <footer className="sticky flex h-16 items-center gap-4 border-dashed border-t bg-background px-4 md:px-6">
                <div className="flex w-full items-center gap-4 md:ml-auto md:gap-2 lg:gap-4">
                    <div className="ml-auto mr-auto flex-1 sm:flex-initial">
                        <div className="flex items-center gap-2 text-lg md:text-base">
                            <Link to={LINK_GITHUB} className="font-semibold hover:underline">github</Link>
                            |
                            <Link to={LINK_TWITTER} className="font-semibold hover:underline">twitter</Link>
                            |
                            <Link to={LINK_INSTAGRAM} className="font-semibold hover:underline">instagram</Link>                    
                        </div>
                    </div>
                </div>
            </footer>
        </div>
    );
}