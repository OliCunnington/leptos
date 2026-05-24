// for access to <head> tag

//leptos_meta package

// <Title/>
// formatter
//  <Title formatter=|text| format!("{text} - My Awsome Site") />
//      use <Title text="value"/> on routes

// <Link/> inject link element into <head>

// <Stylesheet/> creates a <link rel="stylesheet"> with given href

// <Style> creates <style> with children passed in
//  <Style>{include_str!("my_route.css")}</Style> 

// <Meta/> sets <meta> tags with descriptions and other metadata

// <Script/> injects script block into <head>
// <script> renders inside the <body>


// <Html/> and <Body/> allow arbitrary addition of attributes to <html> and <body>
// use spread {..} followed by attributes
//  <Html
//     {..}
//     lang="he"
//     dir="rtl"
//     data-theme="dark"
//  />
