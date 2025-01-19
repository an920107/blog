import AnimatedMark from "@/lib/home/framework/ui/AnimatedMark";

export default function Motto() {
  return (
    <div className="mx-auto flex h-screen max-w-screen-xl flex-col items-center justify-center gap-y-2.5 px-4 md:gap-y-8 md:px-6">
      <div className="flex w-[19rem] flex-col gap-y-3 text-3xl font-bold text-gray-800 md:w-[38rem] md:gap-y-4 md:text-6xl">
        <div className="flex w-full flex-row items-center justify-start gap-x-2.5">
          <span>Keep</span>
          <AnimatedMark text="Learning" direction="left" />
        </div>
        <div className="flex w-full flex-row items-center justify-end gap-x-2.5 md:gap-x-4">
          <AnimatedMark text="Keep" direction="right" />
          <span>Progressing</span>
        </div>
      </div>
    </div>
  );
}
