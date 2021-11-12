const requestURL = 'https://byui-cit230.github.io/lessons/lesson-09/data/latter-day-prophets.json';

fetch(requestURL)
  .then(function (response) {
    return response.json();
  })
  .then(function (jsonObject) {
    //temporary checking for valid response and data parsing
    // console.table(jsonObject);  
    const prophets = jsonObject['prophets'];
    for (let i = 0; i < prophets.length; i++ ) {
        let card = document.createElement('section');
        let h2 = document.createElement('h2');
        let birthdate = document.createElement('h4');
        let birthplace = document.createElement('h4');
        let img = document.createElement('img');

        h2.textContent = prophets[i].name + ' ' + prophets[i].lastname;
        birthdate.textContent = 'Date of Birth: ' + prophets[i].birthdate;
        birthplace.textContent = 'Birth Place: ' + prophets[i].birthplace;
        img.setAttribute('src', prophets[i].imageurl);

        card.appendChild(h2);
        card.appendChild(birthdate);
        card.appendChild(birthplace)
        card.appendChild(img);

        document.querySelector('div.cards').appendChild(card);}
  });

