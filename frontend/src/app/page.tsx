import AboutMe from "@/lib/home/framework/ui/AboutMe";
import FirstView from "@/lib/home/framework/ui/FirstView";
import Motto from "@/lib/home/framework/ui/Motto";

export default function HomePage() {
  return (
    <div>
      <FirstView />
      <AboutMe />
      <Motto />
    </div>
  );
}
