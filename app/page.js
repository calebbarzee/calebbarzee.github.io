import Image from "next/image";

export default function Home() {
  // Load in text from public folder to display ascii wave on homepage

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="font-monospace">
        <h1 className="text-3xl">Simple Solutions, Big Impact</h1>
        <br />
        <p className="text-md">Hi there! I'm Caleb, an engineer who loves to solve big problems with simple code.</p>
        <br />
        <br />
        <h3 className="text-2xl">A Bit About Me</h3>
        <p className="text-sm">I've always been curious about how things work, leading me from playful explorations in my backyard to becoming a passionate software engineer. When I'm not coding, you can find me getting my hands dirty with pottery or gardening, cycling, expressing thoughts through design, or spending quality time with my family. These passions fuel and inspire my work.</p>
        <br />
        <h3 className="text-2xl">My Work</h3>
        <p className="text-sm">As a specialist in Full-Stack Development, I focus on making things work better for everyone. Whether it's using Rust for its reliability or dabbling in JavaScript, Python, C++, and Swift, I pick the best tool for the job.</p>
        <p className="text-sm">
        My goal? To build intuitive digital solutions that have real world impact.</p>
        <br />
        <h3 className="text-2xl">Making a Difference</h3>
        <p className="text-sm">I've helped make marketplaces more trustworthy, city services more accessible, and students learn programming more effectively. My work is all about creating something that matters, reducing complexity, and improving lives.</p>
        <br />
        <h3 className="text-2xl">Let's Chat</h3>
        <p className="text-sm">If you're into creating tech that helps people or just want to share your latest DIY project, I'd love to hear from you.</p>
        <br />
        <ul>
          <li><a href="https://github.com/calebbarzee">My Work -&gt;</a></li>
          <li><a href="mailto:barzeec@gmail.com">Contact Me  -&gt;</a></li>
          <li><a href="https://linkedin.com/in/calebbarzee">Connect -&gt;</a></li>
        </ul>

      </div>
    </main>
  );
}