const YEAR = new Date().getFullYear()

export default {
  footer: (
    <small style={{ display: 'block', marginTop: '8rem' }}>
      <style jsx>{`
        span {
          display: block;
          text-align: center;
          margin: 1rem auto;
        }
        h4 {
          font-size: 0.9rem;
          display: inline;
          margin: 0 2rem 0 0;
        }
        .copyright {
          float: left;
        }
      `}</style>
      <div className="footer">
        <span className="socials">
        <h4>LinkedIn - <a href="https://linkedin.com/in/calebbarzee">calebbarzee</a></h4>
        <h4>GitHub - <a href="https://github.com/calebbarzee">calebbarzee</a></h4>
        <h4>Email - <a href="mailto:barzeec@gmail.com">barzeec@gmail.com</a></h4>
        </span>
        <span className="copyright"><time>{YEAR}</time> © Caleb Barzee</span>
      </div>
    </small>

  )
}
