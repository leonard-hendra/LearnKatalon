<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_DonasiGalang DanaGalang DanaLoginGalan_f621c3</name>
   <tag></tag>
   <elementGuidId>8f3a7a67-57a3-4e94-90f8-c6ffea545f15</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body.modal-open</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>5d98b374-88c5-4a25-b182-a0329e3fc52f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>modal-open</value>
      <webElementGuid>b10f63b4-9927-4289-8760-018d8e9451f7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

	
		
			
				
					
						
							
								
									
									
									
										
									
									Donasi
								
							
						
						
															
									
										
											
											
										
										Galang Dana
									
								
													
						
					
				
			
			
				
					
						
					
				
			
			
				
					
						
							Galang Dana
						
					
				
				
					
												
															
									
										
											
											
										
										Login
									
								
								
									
										
											
											
										
										Galang Dana
									
								
													
						
							
								
									
									
									
										
									
									Donasi
								
							
						
					
				
				
					
						
							
								
								
									
										
									
								
							
						
					
								
			
		
	



	
		
			Beranda
		
		
			Tentang Dana Everyday
		
		
			Semua Campaign
		
		
			Kategori Campaign
		
		
			Hubungi Kami
		
		
			Syarat Dan Ketentuan
		
		
		
							Galang Dana
					
		
					
		
					
			




	
	  
	    
	    
	      
	        
	        
		        Selamat Datang!
		        Mohon masukkan email Anda
		    
	      
	      
	      		      		      		      	
	      		
	      			
		              	
		                	
		                		Email
		                		
		                	
		                
		            
	      		
	      		
	      			
		              	
		                	
		                		Password
		                		
		                	
		                
		            
	      		
				
					
						
							
								Lupa password
							
							|
							
								Daftar akun
								
							
												
					
					
			            
			                LOGIN
			            
			        
				
	        
	      
	    

	  
	


	
	  
	    
	    
	      
	      	
	        
	        
		        Selamat Datang!
		        Mohon masukkan email Anda
		    
	      
	      
	      		      	
	      		
	      			
		              	
		                	
		                		Email
		                		
		                	
		                
		            
		        
				
					
			            
			            	
			                SUBMIT
			            
			        
				
	        
	      
	    

	  
	



  
    
    
      
      	
        
        
	        Selamat Datang!
	        Mohon mengisi formulir pendaftaran
	    
      
      
      	      	
      		
      			
	              	
	                	
	                		First Name
	                		
	                	
	                
	            
	            
	            	
	                	
	                		Last Name
	                		
	                	
	                
	            
	        
	        
      			
      				Gender
	              	
                		
                			
                			Laki-Laki
                		
                		
	                		
		                	Perempuan
		                
	                
	            
	        
	        
      			
	              	
	                	
	                		Phone
	                		
	                	
	                
	            
	            
	              	
	                	
	                		Email
	                		
	                	
	                
	            
	        
	        
      			
	              	
	                	
	                		Date Of Birth
	                		
	                	
	                
	            
	        
	        
      			
	              	
	                	
	                		Password
	                		
	                	
	                
	            
	            
	              	
	                	
	                		Confirm Password
	                		
	                	
	                
	            
	        
	        
	        	
					
				      	
				            
			              	
				        
				    
				
				
			   		REGISTER
			   	
			
			
				
					Sudah punya akun? Click disini untuk login
				
			
        
      
    

  


				$(document).ready(function(){
		$('.angka').keyup(function () {  
	  		this.value = this.value.replace(/[^0-9.]/g,'');
		});
		$('.email-val').keypress(function (event) {
	      var inputValue = event.charCode;

	      //NOTE: in firefox the backspace is keycoded as 0
	        if(!((inputValue > 64 &amp;&amp; inputValue &lt; 91) || (inputValue > 96 &amp;&amp; inputValue &lt; 123)||(inputValue==64)||(inputValue==46)||(inputValue==95)||(inputValue==45)||(inputValue > 47 &amp;&amp; inputValue &lt; 58) || inputValue==0))
	        {
	            event.preventDefault();
	        }
	    });
	    $('.charNum').keypress(function (event) {
	      var inputValue = event.charCode;
	        if(!((inputValue > 64 &amp;&amp; inputValue &lt; 91) || (inputValue > 96 &amp;&amp; inputValue &lt; 123)||(inputValue==32)||(inputValue > 47 &amp;&amp; inputValue &lt; 58) || inputValue==0))
	        {
	            event.preventDefault();
	        }
	    });
	    $('.charOnly').keypress(function (event) {
	      var inputValue = event.charCode;
	        if(!((inputValue > 64 &amp;&amp; inputValue &lt; 91) || (inputValue > 96 &amp;&amp; inputValue &lt; 123)||(inputValue==32) || inputValue==0))
	        {
	            event.preventDefault();
	        }
	    });
					[].slice.call( document.querySelectorAll( '.form-control')).forEach( function( inputEl) {
				if( inputEl.value.trim() !== '' ) {
					
					// $(ev.target.parentNode).addClass('filled-text' );
				}
				inputEl.addEventListener( 'focus', onInputFocus );
				inputEl.addEventListener( 'blur', onInputBlur );
			});

			function onInputFocus( ev ) {
				$(ev.target.parentNode).addClass('filled-text');
			}

			function onInputBlur( ev ) {
				if( ev.target.value.trim() === '' ) {
					$(ev.target.parentNode).removeClass('filled-text');
				}
			}
		
		// $('#regisPop').show();
	  	// $('#regisPop').removeClass('fade');
	});
	function showlogin(){
	  $('#loginPop').show();
	  $('#loginPop').addClass('in');
	  $('#sliding-menu').hide();
	  $('body').addClass('modal-open');
  	  $('.modal-backdrop.in').remove();
	  $('#regisPop').hide();
	}
	function showforgot(){
	  $('#loginPop').modal('hide');
	  setTimeout(function(){

	  $('body').addClass('modal-open');
	  $('#forgotPop').modal('show');
	  },500);
	}
	function showregis(){
  	  $('#loginPop').modal('hide');
	  setTimeout(function(){

	  $('body').addClass('modal-open');
	  $('#regisPop').modal('show');
	  },500);
	}
	function closeModal(){
	  $('.modal').hide();
	  $('.modal').removeClass('in');
  	  $('.modal-backdrop.in').remove();
	  $('#sliding-menu').hide();
	  $('body').removeClass('modal-open');
	}
	function back(){
	  $('#forgotPop').modal('hide');
	  $('#regisPop').modal('hide');
	  setTimeout(function(){

	  $('body').addClass('modal-open');
	  $('#loginPop').modal('show');
	  },500);
	}


//validate password
  var password = document.getElementById(&quot;member_password&quot;)
  , confirm_password = document.getElementById(&quot;confirm_password&quot;);

  function validatePassword(){
    if(password.value != confirm_password.value) {
      confirm_password.setCustomValidity(&quot;Konfirmasi kata sandi tidak cocok!&quot;);
    } else {
      confirm_password.setCustomValidity('');
    }
  }

  password.onchange = validatePassword;
  confirm_password.onkeyup = validatePassword;

  $(document).ready(function(){
    $(&quot;#register_form&quot;).validate({
          rules: {
           member_password: &quot;required&quot;,
           member_first_name: &quot;required&quot;,
           member_birthdate: &quot;required&quot;,
           member_phone: &quot;required&quot;,
           member_email: {
           	required:true,
           	email:true,
           	remote:
           	{
           		url: &quot;https://www.danaeveryday.id/member/register_email_exists&quot;,
           		type: &quot;post&quot;,
           		data:{
           			email: function(){ return $(&quot;.email-regis&quot;).val(); }
           		}
           	}
           },
           &quot;hiddenRecaptcha1&quot;: {
                 required: function() {
                     if(grecaptcha.getResponse() == '') {
                         return true;
                     } else {
                         return false;
                     }
                 }
             }
        },
          messages: {
           required: &quot;This Field is Required&quot;,
           member_email: {
           	email: &quot;Email is not valid.&quot;,
           	remote: &quot;Email has been used.&quot;
           },
           &quot;hiddenRecaptcha1&quot;: &quot;Captcha is required&quot;
        }
     });
    $(&quot;#login_form&quot;).validate({
          rules: {
           email: {
           	required:true,
           	email:true
           },
           member_password: &quot;required&quot;
        },
          messages: {
           required: &quot;This Field is Required&quot;,
           email: &quot;Email is not valid&quot;
        }
    });
    $(&quot;#forgot_form&quot;).validate({
          rules: {
           member_email: {
           	required:true,
           	email:true
           }
        },
          messages: {
           required: &quot;This Field is Required&quot;,
           email: &quot;Email is not valid&quot;
        }
    });
  })



	
	
    	
        
        
            
                Bersama Dana Everyday Kita Melestarikan Buddha Sasana di Nusantara &amp; Dunia
                				Yuk, Download!
				
				
                                    
            
			 
                
                    
                
            
        
    

		
			
				
					
						
							210 
							Campaign Terdanai
						
					
					
						
							Rp 39.032.330.317 
							Donasi Tersalurkan
						
					
					
						
							5.507 
							Donatur Tergabung
						
					
				

				
					
						
							
								210 
								Campaign Terdanai
							
						
						
							
								5.507 
								Donatur Tergabung
							
						
					
					
						
							
								Rp 39.032.330.317 
								Donasi Tersalurkan
							
						
					
				

			
		
	

	
	
		
			
				
					
						
							Semua Kategori
															Pembangunan &amp; Renovasi Vihara
															Pendidikan Buddhist
															Sangha Dana
															Sosial Kemanusiaan
															Ashoka Spirit
															Fangshen
							                      	
					
					
						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pembangunan Pagar Vihara Dhammadipa...
									
										Vihara Dhammad?pa loka di bangun pada tahun 1989, pembangunan vihara ini merupakan partisipasi umat Buddha s...									
									
									
										
																						64 hari  sisa waktu
																					
									
									
										4%  tercapai
									
									
										Rp 3.923.101  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Pembangunan PUSDIKLAT Sikkhadama Mahamet...
									
										    PUSDIKLAT Sikkhadama Mahametta Palangkaraya adalah PUSDIKLAT Sangha Theravada Indonesia pertama di P...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										46%  tercapai
									
									
										Rp 459.147.258  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Bangun Kuti Arama Kebun Kesadaran d...
									
										Arama Kebun Kesadaran adalah Salah satu tempat ibadah Agama Buddha di
Provinsi Sulawesi Utara dan merupakan...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										57%  tercapai
									
									
										Rp 276.724.892  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pendidikan Buddhist
									Dana Guru Bahasa &amp; Coding Pesastrian...
									
										Yayasan Kusalamitra sudah membantu anak-anak kurang mampu di berbagai daerah di Indonesia untuk dapat melanj...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										84%  tercapai
									
									
										Rp 69.477.452  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Sangha Dana
									Dana Kendaraan Kepada Sangha untuk Pemba...
									
										Sangha dalam pembabaran Dhamma memerlukan sarana yang dapat menunjang dalam mencapai Vihara-vihara yang jauh...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										34%  tercapai
									
									
										Rp 61.349.392  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana 5 Bidang Relief Buddha di Vihara Sa...
									
										Vihara Sangupati yang dibangun oleh tokoh Buddhis setempat pad tahun 1972 hancur karena gepa Lombok tahun 20...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										31%  tercapai
									
									
										Rp 45.041.458  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pembangunan Lantai Kuti &amp; Pendo...
									
										Ada 3 faktor yang menentukan besarnya jasa kebajikan yang diperoleh dari berdana, yaitu: sifat dari motif pe...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										21%  tercapai
									
									
										Rp 81.828.136  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pembangunan Kuti Vihara Samyag Dars...
									
										Vihara Samyag Darsana kini telah berusia 43 tahun. Ia didirikan dengan semangat
gotong royong oleh umat Bud...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										43%  tercapai
									
									
										Rp 120.361.628  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pemagaran Vihara Giri Ratana Puja L...
									
										Umat Buddha Vihara Giri Ratana Puja Lombok makin menunjukan perkembangan dalam mempelajari dan mempraktekan ...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										77%  tercapai
									
									
										Rp 197.288.160  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pengerjaan Pondasi &amp; Dinding Sa...
									
										Vihara Metta Mandala berdiri sejak tahun 1986 dengan jumlah umat 32 KK (120 Jiwa). Vihara Metta Mandala bera...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										51%  tercapai
									
									
										Rp 84.144.143  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Renovasi Vihara Sangha Metta Arama ...
									
										Vihara Sangha Metta Arama adalah salah satu tempat ibadah umat Buddha aliran Theravada yang bertempat di
Jl...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										84%  tercapai
									
									
										Rp 124.651.099  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pengerjaan Lantai Vihara Vidya Loka...
									
										Vihara Vidya Loka terletak di Dusun Ngroto RT RW 008 005 Desa Sumogawe Kecamatan Getasan Kabupaten Semarang ...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										40%  tercapai
									
									
										Rp 71.183.982  terkumpul
									
												
							
						

											
					Load More
					
						$(document).ready(function() {
							var total_campaigns = 18;
							$('#campaign-load-more').on('click', function(event) {
								event.preventDefault();
								
								$.ajax({
						            url: 'https://www.danaeveryday.id/campaign_category/getCampaignFromCategory',
						            dataType: 'json',
						            data: {
						                    category_id: $('.category-list.current a').data('category'),
						                    offset: $('#campaign-load-more').attr('data-offset')
						                  },
						            beforeSend: function() {
						            	$('.campaign-crit').append('&lt;div class=&quot;abs-loading&quot;>&lt;div class=&quot;spinner-loading&quot;>&lt;/div>&lt;/div>');
						            },
						            complete: function(json) {
						            	json = json.responseJSON;

						            	// clear campaign list
						            	// $('.campaign-list').html('');
						            	$('#campaign-load-more').attr('data-offset', parseInt($('#campaign-load-more').attr('data-offset')) + 3);
						            	if (typeof json['error'] === 'undefined') {
							            	// append campaign
							            	for (var i = 0; i &lt; json.length; i++) {
							            		var donation_collected = json[i][&quot;donation_collected&quot;] ? json[i][&quot;donation_collected&quot;] : 0,
							            			percent = json[i][&quot;percent&quot;],
							            			html = '\
							            				&lt;div class=&quot;col inline l-4 m-6 s-6 min-10 item-sponsor&quot;>\
														&lt;a href=&quot;https://www.danaeveryday.id/campaign/'+ json[i][&quot;seo_url&quot;] +'&quot;>\
															&lt;div class=&quot;hvr-sponsor&quot;>\
																&lt;div class=&quot;img-sponsor&quot;>\
																	&lt;img src=&quot;'+ json[i][&quot;image&quot;] +'&quot; alt=&quot;'+ json[i][&quot;image_alt&quot;] +'&quot;/>\
																&lt;/div>\
															&lt;/div>\
															&lt;div class=&quot;pad-sponsor&quot;>\
																&lt;div class=&quot;tag-sponsor&quot;>&lt;i class=&quot;fa fa-tags&quot; aria-hidden=&quot;true&quot;>&lt;/i>'+ json[i][&quot;categories_name&quot;] +'&lt;/div>\
																&lt;div class=&quot;n-sponsor&quot;>'+ json[i][&quot;name&quot;] +'&lt;/div>\
																&lt;div class=&quot;bdy-sponsor&quot;>'+ limitString(json[i][&quot;description&quot;]) +'&lt;/div>\
																&lt;div class=&quot;bar-sponsor&quot;>&lt;div class=&quot;in-bar-sponsor&quot; style=&quot;width: '+ percent +'%;&quot;>&lt;/div>&lt;/div>\
																&lt;div>\
																	&lt;div class=&quot;d-sponsor&quot;>'+ diffDate('2024-11-28', json[i][&quot;end_date&quot;]) +' hari &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>\
																&lt;/div>\
																&lt;div>\
																	&lt;div class=&quot;d-sponsor&quot;>'+ percent +'% &lt;span class=&quot;o-sponsor&quot;> tercapai&lt;/span>&lt;/div>\
																&lt;/div>\
																&lt;div>\
																	&lt;div class=&quot;d-sponsor&quot;>Rp '+ thousandFormat(donation_collected) +' &lt;span class=&quot;o-sponsor&quot;> terkumpul&lt;/span>&lt;/div>\
																&lt;/div>\
															&lt;/div>\
														&lt;/a>\
													&lt;/div>\
							            			';

							            		// add campaign
							            		$('.campaign-list').append(html);

							            		// scroll to
							            		var $container = $(&quot;html,body&quot;);
												var $scrollTo = $('.campaign-crit');

												$container.animate({scrollTop: $scrollTo.offset().top, scrollLeft: 0},300);
							            	}
							            }
							            else {
							            }
							            	if ($('.campaign-list .item-sponsor').length >= total_campaigns) {
							            		$('#campaign-load-more').addClass('hidden');
							            	}

						        		// remove loading
						        		setTimeout(function() {
						        			$('.abs-loading').remove();
						        		}, 500);
						            },
						        });
							});
						});
					
				
				
					
						Semua Kategori
													Pembangunan &amp; Renovasi Vihara
													Pendidikan Buddhist
													Sangha Dana
													Sosial Kemanusiaan
													Ashoka Spirit
													Fangshen
											
				
			
		
	
	
		
			
				
					
						
					
				
				
					Dana Everyday mempertemukan ladang berdana yang subur kepada para donatur, seperti: 
					
						
							Dana pembangunan vihara-vihara desa.
							Dana cinta kasih pengobatan umat Buddha yang kurang mampu.
							Baksos untuk umat Buddha yang kurang mampu.
							Dana makan, jubah, keperluan Sangha.
							Visudhi Tisarana.
							Dan masih banyak lagi.
						
					
				
			
		
	
				
		
			
				
				Latest Campaign
				Pilih dan salurkan donasi untuk campaign yang berarti bagi Anda.
				
					
					
						
							
								
									
								
							
							
								Dana Pembangunan Pagar Vihara Dhammadipa...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 3.923.101
										
									
									
										
											Tercapai
											4%
										
									
									
										
											Sisa Waktu
											
																									64 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Pembangunan PUSDIKLAT Sikkhadama Mahamet...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 459.147.258
										
									
									
										
											Tercapai
											46%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Bangun Kuti Arama Kebun Kesadaran d...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 276.724.892
										
									
									
										
											Tercapai
											57%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Guru Bahasa &amp; Coding Pesastrian...
								
									
									Pendidikan Buddhist								
								
								
									
										
											Terkumpul
											Rp 69.477.452
										
									
									
										
											Tercapai
											84%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Kendaraan Kepada Sangha untuk Pemba...
								
									
									Sangha Dana								
								
								
									
										
											Terkumpul
											Rp 61.349.392
										
									
									
										
											Tercapai
											34%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana 5 Bidang Relief Buddha di Vihara Sa...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 45.041.458
										
									
									
										
											Tercapai
											31%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pembangunan Lantai Kuti &amp; Pendo...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 81.828.136
										
									
									
										
											Tercapai
											21%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pembangunan Kuti Vihara Samyag Dars...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 120.361.628
										
									
									
										
											Tercapai
											43%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pemagaran Vihara Giri Ratana Puja L...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 197.288.160
										
									
									
										
											Tercapai
											77%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pengerjaan Pondasi &amp; Dinding Sa...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 84.144.143
										
									
									
										
											Tercapai
											51%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Renovasi Vihara Sangha Metta Arama ...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 124.651.099
										
									
									
										
											Tercapai
											84%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pengerjaan Lantai Vihara Vidya Loka...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 71.183.982
										
									
									
										
											Tercapai
											40%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

										
				
				See All Projects
			
		
	
	
    	
    	    Partners
    	    
    	        
    	            
    	                
    	                
    	       
    	        
    	            MLIKI is a limitless digital item and physical goods marketplace. MLIKI aims to create a hybrid marketplace that makes it possible to purchase physical goods and digital item like NFTs at the touch of a finger
                    
                    
                    
    	        
    	    
    	
	



$('.slider-banner').slick({
	autoplay: true,
	autoplaySpeed: 2000,
	arrows: true,
	dots: false,
	slidesToShow: 1,
	slidesToScroll: 1,
});

$('.slider-campaign').slick({
	// autoplay: true,
	// autoplaySpeed: 2000,
	// arrows: true,
	// dots: false,
	// slidesToShow: 1,
	// slidesToScroll: 1,
	// centerMode: true
	prevArrow: '&lt;i class=&quot;fa fa-caret-left campaign-control tY l40-md l20-xs&quot; aria-hidden=&quot;true&quot;>&lt;/i>',
    nextArrow: '&lt;i class=&quot;fa fa-caret-right campaign-control tY r40-md r20-xs&quot; aria-hidden=&quot;true&quot;>&lt;/i>',
    responsive: [
      {
        breakpoint: 767,
        settings: {
    		adaptiveHeight: true
        }
      }
    ]
});

$(&quot;.slider-banner&quot;).on(&quot;afterChange&quot;, function (event, slick, currentSlide, nextSlide){
	var seo = 'https://www.danaeveryday.id/campaign/' + $(this).find('.item[data-slick-index=&quot;'+ currentSlide +'&quot;]').data('seo');

	$('.featured-link').attr('href', seo);
})

// $('ul.l-sponsor li').click(function(){
// 	var tab_id = $(this).attr('data-tab');

// 	$('ul.l-sponsor li').removeClass('current');
// 	$('.tab-content').removeClass('current');

// 	$(this).addClass('current');
// 	$(&quot;.&quot;+tab_id).addClass('current');
// });

// $(document).on('change', '.tab-select', function(){
// 	console.log($(this).find('option:selected').data('tab'));
// 	var tab_id = $(this).find('option:selected').data('tab');

// 	$('ul.l-sponsor li').removeClass('current');
// 	$('.tab-content').removeClass('current');

// 	if (tab_id != 'all') {
// 		$(&quot;.tab-&quot;+tab_id).addClass('current');
// 	}
// 	else {
// 		$('.tab-content').addClass('current');
// 	}
// });

$('ul.l-sponsor li.tab-all').click(function(){
	$('.tab-content').addClass('current');
});

function check_sticky() {
	$(&quot;[data-sticky_column]&quot;).stick_in_parent({
		offset_top: 85,
 		parent: &quot;[data-sticky_parent]&quot;
   	}).on(&quot;sticky_kit:stick&quot;, function(e) {
   		$('.sticky').addClass('stuck');
	}).on(&quot;sticky_kit:unstick&quot;, function(e) {
   		$('.sticky').removeClass('stuck');
  	});
}
$(document).ready(function() {
	check_sticky();
});

$(document).ready(function() {
	// ajax categories
	$(document).on('click', '.category-list a', function(event) {
		event.preventDefault();
		var $this = $(this);

		$('ul.l-sponsor li').removeClass('current');
		$this.parent().addClass('current');

		$.ajax({
            url: 'https://www.danaeveryday.id/campaign_category/getCampaignFromCategory',
            dataType: 'json',
            data: {
                    category_id: $(this).data('category')
                  },
            beforeSend: function() {
            	$('.campaign-crit').append('&lt;div class=&quot;abs-loading&quot;>&lt;div class=&quot;spinner-loading&quot;>&lt;/div>&lt;/div>');

            	// dont scroll
            	// $('body').on({
				//     'mousewheel': function(e) {
				//         e.preventDefault();
				//     }
				// });
            },
            complete: function(json) {
            	json = json.responseJSON;

            	// clear campaign list
            	$('.campaign-list').html('');

            	if (typeof json['error'] === 'undefined') {
	            	// append campaign
	            	for (var i = 0; i &lt; json.length; i++) {
	            		var donation_collected = json[i][&quot;donation_collected&quot;] ? json[i][&quot;donation_collected&quot;] : 0,
	            			percent = json[i][&quot;percent&quot;],
	            			html = '\
	            				&lt;div class=&quot;col inline l-4 m-6 s-6 min-10 item-sponsor&quot;>\
								&lt;a href=&quot;https://www.danaeveryday.id/campaign/'+ json[i][&quot;seo_url&quot;] +'&quot;>\
									&lt;div class=&quot;hvr-sponsor&quot;>\
										&lt;div class=&quot;img-sponsor&quot;>\
											&lt;img src=&quot;'+ json[i][&quot;image&quot;] +'&quot; alt=&quot;'+ json[i][&quot;image_alt&quot;] +'&quot;/>\
										&lt;/div>\
									&lt;/div>\
									&lt;div class=&quot;pad-sponsor&quot;>\
										&lt;div class=&quot;tag-sponsor&quot;>&lt;i class=&quot;fa fa-tags&quot; aria-hidden=&quot;true&quot;>&lt;/i>'+ json[i][&quot;categories_name&quot;] +'&lt;/div>\
										&lt;div class=&quot;n-sponsor&quot;>'+ json[i][&quot;name&quot;] +'&lt;/div>\
										&lt;div class=&quot;bdy-sponsor&quot;>'+ limitString(json[i][&quot;description&quot;]) +'&lt;/div>\
										&lt;div class=&quot;bar-sponsor&quot;>&lt;div class=&quot;in-bar-sponsor&quot; style=&quot;width: '+ percent +'%;&quot;>&lt;/div>&lt;/div>\
										&lt;div>';
							if (json[i]['date_type'] == 2) {
								html += '&lt;div class=&quot;d-sponsor&quot;>&lt;img src=&quot;' + base_url + 'lib/images/icons/unlimited.png&quot; style=&quot;width:20px;&quot;> &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>';
							} else {
								html += '&lt;div class=&quot;d-sponsor&quot;>'+ diffDate('2024-11-28', json[i][&quot;end_date&quot;]) +' hari &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>';
							}
							html +=		'&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>'+ percent +'% &lt;span class=&quot;o-sponsor&quot;> tercapai&lt;/span>&lt;/div>\
										&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>Rp '+ thousandFormat(donation_collected) +' &lt;span class=&quot;o-sponsor&quot;> terkumpul&lt;/span>&lt;/div>\
										&lt;/div>\
									&lt;/div>\
								&lt;/a>\
							&lt;/div>\
	            			';

	            		// add campaign
	            		$('.campaign-list').append(html);

	            		// scroll to
	            		var $container = $(&quot;html,body&quot;);
						var $scrollTo = $('.campaign-crit');

						$container.animate({scrollTop: $scrollTo.offset().top, scrollLeft: 0},300);
	            	}
	            }
	            else {
	            	$('.campaign-list').append('&lt;div class=&quot;col inline&quot;>&lt;div>Campaign Belum Tersedia&lt;/div>&lt;/div>');
	            }

        		// remove loading
        		setTimeout(function() {
        			// unbind scroll
        			// $('body').on({
					//     'mousewheel': function(e) {
					//         $(this).unbind();
					//     }
					// });

        			$('.abs-loading').remove();
        		}, 500);
            },
        });
	});

	$(document).on('change', '.tab-select', function(event) {
		event.preventDefault();
		var cat = $(this).val(),
			$this = $('a[data-category='+ cat +']');

		$('ul.l-sponsor li').removeClass('current');
		$this.parent().addClass('current');

		$.ajax({
            url: 'https://www.danaeveryday.id/campaign_category/getCampaignFromCategory',
            dataType: 'json',
            data: {
                    category_id: $(this).val()
                  },
            beforeSend: function() {
            	$('.campaign-crit').append('&lt;div class=&quot;abs-loading&quot;>&lt;div class=&quot;spinner-loading&quot;>&lt;/div>&lt;/div>');

            	// dont scroll
            	// $('body').on({
				//     'mousewheel': function(e) {
				//         e.preventDefault();
				//     }
				// });
            },
            complete: function(json) {
            	json = json.responseJSON;
				
            	// clear campaign list
            	$('.campaign-list').html('');

            	if (typeof json['error'] === 'undefined') {
	            	// append campaign
	            	for (var i = 0; i &lt; json.length; i++) {
	            		var percent = Math.ceil((json[i][&quot;donation_collected&quot;]/json[i][&quot;donation&quot;]) * 100),
	            			html = '\
	            				&lt;div class=&quot;col inline l-4 m-6 s-6 min-10 item-sponsor&quot;>\
								&lt;a href=&quot;https://www.danaeveryday.id/campaign/'+ json[i][&quot;seo_url&quot;] +'&quot;>\
									&lt;div class=&quot;hvr-sponsor&quot;>\
										&lt;div class=&quot;img-sponsor&quot;>\
											&lt;img src=&quot;'+ json[i][&quot;image&quot;] +'&quot; alt=&quot;'+ json[i][&quot;image_alt&quot;] +'&quot;/>\
										&lt;/div>\
									&lt;/div>\
									&lt;div class=&quot;pad-sponsor&quot;>\
										&lt;div class=&quot;tag-sponsor&quot;>&lt;i class=&quot;fa fa-tags&quot; aria-hidden=&quot;true&quot;>&lt;/i>'+ json[i][&quot;categories_name&quot;] +'&lt;/div>\
										&lt;div class=&quot;n-sponsor&quot;>'+ json[i][&quot;name&quot;] +'&lt;/div>\
										&lt;div class=&quot;bdy-sponsor&quot;>'+ limitString(json[i][&quot;description&quot;]) +'&lt;/div>\
										&lt;div class=&quot;bar-sponsor&quot;>&lt;div class=&quot;in-bar-sponsor&quot; style=&quot;width: '+ percent +'%;&quot;>&lt;/div>&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>'+ diffDate('2024-11-28', json[i][&quot;end_date&quot;]) +' hari &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>\
										&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>'+ percent +'% &lt;span class=&quot;o-sponsor&quot;> tercapai&lt;/span>&lt;/div>\
										&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>Rp '+ thousandFormat(json[i][&quot;donation_collected&quot;]) +' &lt;span class=&quot;o-sponsor&quot;> terkumpul&lt;/span>&lt;/div>\
										&lt;/div>\
									&lt;/div>\
								&lt;/a>\
							&lt;/div>\
	            			';

	            		// add campaign
	            		$('.campaign-list').append(html);
	            	}
	            }
	            else {
	            	$('.campaign-list').append('&lt;div class=&quot;col inline&quot;>&lt;div>Campaign Belum Tersedia&lt;/div>&lt;/div>');
	            }

        		// remove loading
        		setTimeout(function() {
        			// unbind scroll
        			// $('body').on({
					//     'mousewheel': function(e) {
					//         $(this).unbind();
					//     }
					// });

        			$('.abs-loading').remove();
        		}, 500);
            },
        });
	});
});


	
		
			
				
					
						
							
							Home
						
					
				
				
					
						
							
							Donasi
						
					
				
				
					
												
													
							
															Login
														
						
					
				
				
					
						
							
							Menu
						
					
				
			
		
	
	
		
			
				
					
						
					
				
				
					
						
							TAKE ACTION
							
								Seluruh Campaign
															Galang Dana
														
						
						
							LEARN MORE
							
								Apa itu Dana Everyday?
								Hubungi Kami
								Syarat Dan Ketentuan
							
						
						
							CONNECT
														
								
							
														
								
							
													
					
					
						Copyright 2024 Yayasan Dana Everyday (DEV). All Rights Reserved.
						
						    Gositus
						
					
				
			
		
	



var title = &quot;Home&quot;;
var url = &quot;&quot;;
var url2 = &quot;&quot;;
var recaptcha3;
var recaptcha1;
var recaptcha2;
var recaptcha4;
var recaptcha5;
var recaptcha6;
var recaptcha7;

window.verifyCallback3 = function(response) {
    // return response;
	$('#hiddenRecaptcha3').prop('value', response);
	console.log(response)
};
window.verifyCallback1 = function(response) {
	// return response;
	$('#hiddenRecaptcha1').prop('value', response);
	console.log(response)
};
if(title == 'Reset Password'){
	window.verifyCallback4 = function(response) {
		// return response;
		$('#hiddenRecaptcha4').prop('value', response);
		console.log(response)
	};
}
if(title == 'Contact Us'){
	window.verifyCallback5 = function(response) {
		// return response;
		$('#hiddenRecaptcha5').prop('value', response);
		console.log(response)
	};
}
if(title == 'Edit Account'){
	window.verifyCallback6 = function(response) {
		// return response;
		$('#hiddenRecaptcha6').prop('value', response);
		console.log(response)
	};
}
if(url == 'campaign' &amp;&amp; url2 != ''){
	window.verifyCallback7 = function(response) {
		// return response;
		$('#hiddenRecaptcha7').prop('value', response);
		console.log(response)
	};
}
var myCallBack = function() {
    recaptcha1 = grecaptcha.render('recaptcha1', {
      'sitekey' : '6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q',
      'theme' : 'light',
      'callback' : verifyCallback1
    });
    if(title == 'Reset Password'){
	    /*recaptcha4 = grecaptcha.render('recaptcha4', {
	      'sitekey' : '6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q', 
	      'theme' : 'light',
	      'callback' : verifyCallback4
	    });*/
	}	
    if(title == 'Contact Us'){
	    recaptcha5 = grecaptcha.render('recaptcha5', {
	      'sitekey' : '6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q', 
	      'theme' : 'light',
	      'callback' : verifyCallback5
	    });
	}
    if(title == 'Edit Account'){
	    /*recaptcha6 = grecaptcha.render('recaptcha6', {
	      'sitekey' : '6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q', 
	      'theme' : 'light',
	      'callback' : verifyCallback6
	    });*/
	}

    if(url == 'campaign' &amp;&amp; url2 != ''){
	    recaptcha7 = grecaptcha.render('recaptcha7', {
	      'sitekey' : '6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q', 
	      'theme' : 'light',
	      'callback' : verifyCallback7
	    });
	}
};

        


  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());

  gtag('config', 'G-9TQY6RNL58');


id(&quot;email&quot;)</value>
      <webElementGuid>53f64690-344e-46ef-ab93-c3b41af62ab0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;modal-open&quot;]</value>
      <webElementGuid>5c605361-ba8c-4f55-b03c-792934e42052</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>b311328c-2c32-48ca-ba33-ef68e70d3f0b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;

	
		
			
				
					
						
							
								
									
									
									
										
									
									Donasi
								
							
						
						
															
									
										
											
											
										
										Galang Dana
									
								
													
						
					
				
			
			
				
					
						
					
				
			
			
				
					
						
							Galang Dana
						
					
				
				
					
												
															
									
										
											
											
										
										Login
									
								
								
									
										
											
											
										
										Galang Dana
									
								
													
						
							
								
									
									
									
										
									
									Donasi
								
							
						
					
				
				
					
						
							
								
								
									
										
									
								
							
						
					
								
			
		
	



	
		
			Beranda
		
		
			Tentang Dana Everyday
		
		
			Semua Campaign
		
		
			Kategori Campaign
		
		
			Hubungi Kami
		
		
			Syarat Dan Ketentuan
		
		
		
							Galang Dana
					
		
					
		
					
			




	
	  
	    
	    
	      
	        
	        
		        Selamat Datang!
		        Mohon masukkan email Anda
		    
	      
	      
	      		      		      		      	
	      		
	      			
		              	
		                	
		                		Email
		                		
		                	
		                
		            
	      		
	      		
	      			
		              	
		                	
		                		Password
		                		
		                	
		                
		            
	      		
				
					
						
							
								Lupa password
							
							|
							
								Daftar akun
								
							
												
					
					
			            
			                LOGIN
			            
			        
				
	        
	      
	    

	  
	


	
	  
	    
	    
	      
	      	
	        
	        
		        Selamat Datang!
		        Mohon masukkan email Anda
		    
	      
	      
	      		      	
	      		
	      			
		              	
		                	
		                		Email
		                		
		                	
		                
		            
		        
				
					
			            
			            	
			                SUBMIT
			            
			        
				
	        
	      
	    

	  
	



  
    
    
      
      	
        
        
	        Selamat Datang!
	        Mohon mengisi formulir pendaftaran
	    
      
      
      	      	
      		
      			
	              	
	                	
	                		First Name
	                		
	                	
	                
	            
	            
	            	
	                	
	                		Last Name
	                		
	                	
	                
	            
	        
	        
      			
      				Gender
	              	
                		
                			
                			Laki-Laki
                		
                		
	                		
		                	Perempuan
		                
	                
	            
	        
	        
      			
	              	
	                	
	                		Phone
	                		
	                	
	                
	            
	            
	              	
	                	
	                		Email
	                		
	                	
	                
	            
	        
	        
      			
	              	
	                	
	                		Date Of Birth
	                		
	                	
	                
	            
	        
	        
      			
	              	
	                	
	                		Password
	                		
	                	
	                
	            
	            
	              	
	                	
	                		Confirm Password
	                		
	                	
	                
	            
	        
	        
	        	
					
				      	
				            
			              	
				        
				    
				
				
			   		REGISTER
			   	
			
			
				
					Sudah punya akun? Click disini untuk login
				
			
        
      
    

  


				$(document).ready(function(){
		$(&quot; , &quot;'&quot; , &quot;.angka&quot; , &quot;'&quot; , &quot;).keyup(function () {  
	  		this.value = this.value.replace(/[^0-9.]/g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		});
		$(&quot; , &quot;'&quot; , &quot;.email-val&quot; , &quot;'&quot; , &quot;).keypress(function (event) {
	      var inputValue = event.charCode;

	      //NOTE: in firefox the backspace is keycoded as 0
	        if(!((inputValue > 64 &amp;&amp; inputValue &lt; 91) || (inputValue > 96 &amp;&amp; inputValue &lt; 123)||(inputValue==64)||(inputValue==46)||(inputValue==95)||(inputValue==45)||(inputValue > 47 &amp;&amp; inputValue &lt; 58) || inputValue==0))
	        {
	            event.preventDefault();
	        }
	    });
	    $(&quot; , &quot;'&quot; , &quot;.charNum&quot; , &quot;'&quot; , &quot;).keypress(function (event) {
	      var inputValue = event.charCode;
	        if(!((inputValue > 64 &amp;&amp; inputValue &lt; 91) || (inputValue > 96 &amp;&amp; inputValue &lt; 123)||(inputValue==32)||(inputValue > 47 &amp;&amp; inputValue &lt; 58) || inputValue==0))
	        {
	            event.preventDefault();
	        }
	    });
	    $(&quot; , &quot;'&quot; , &quot;.charOnly&quot; , &quot;'&quot; , &quot;).keypress(function (event) {
	      var inputValue = event.charCode;
	        if(!((inputValue > 64 &amp;&amp; inputValue &lt; 91) || (inputValue > 96 &amp;&amp; inputValue &lt; 123)||(inputValue==32) || inputValue==0))
	        {
	            event.preventDefault();
	        }
	    });
					[].slice.call( document.querySelectorAll( &quot; , &quot;'&quot; , &quot;.form-control&quot; , &quot;'&quot; , &quot;)).forEach( function( inputEl) {
				if( inputEl.value.trim() !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ) {
					
					// $(ev.target.parentNode).addClass(&quot; , &quot;'&quot; , &quot;filled-text&quot; , &quot;'&quot; , &quot; );
				}
				inputEl.addEventListener( &quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, onInputFocus );
				inputEl.addEventListener( &quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;, onInputBlur );
			});

			function onInputFocus( ev ) {
				$(ev.target.parentNode).addClass(&quot; , &quot;'&quot; , &quot;filled-text&quot; , &quot;'&quot; , &quot;);
			}

			function onInputBlur( ev ) {
				if( ev.target.value.trim() === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ) {
					$(ev.target.parentNode).removeClass(&quot; , &quot;'&quot; , &quot;filled-text&quot; , &quot;'&quot; , &quot;);
				}
			}
		
		// $(&quot; , &quot;'&quot; , &quot;#regisPop&quot; , &quot;'&quot; , &quot;).show();
	  	// $(&quot; , &quot;'&quot; , &quot;#regisPop&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;fade&quot; , &quot;'&quot; , &quot;);
	});
	function showlogin(){
	  $(&quot; , &quot;'&quot; , &quot;#loginPop&quot; , &quot;'&quot; , &quot;).show();
	  $(&quot; , &quot;'&quot; , &quot;#loginPop&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;in&quot; , &quot;'&quot; , &quot;);
	  $(&quot; , &quot;'&quot; , &quot;#sliding-menu&quot; , &quot;'&quot; , &quot;).hide();
	  $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;modal-open&quot; , &quot;'&quot; , &quot;);
  	  $(&quot; , &quot;'&quot; , &quot;.modal-backdrop.in&quot; , &quot;'&quot; , &quot;).remove();
	  $(&quot; , &quot;'&quot; , &quot;#regisPop&quot; , &quot;'&quot; , &quot;).hide();
	}
	function showforgot(){
	  $(&quot; , &quot;'&quot; , &quot;#loginPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
	  setTimeout(function(){

	  $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;modal-open&quot; , &quot;'&quot; , &quot;);
	  $(&quot; , &quot;'&quot; , &quot;#forgotPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
	  },500);
	}
	function showregis(){
  	  $(&quot; , &quot;'&quot; , &quot;#loginPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
	  setTimeout(function(){

	  $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;modal-open&quot; , &quot;'&quot; , &quot;);
	  $(&quot; , &quot;'&quot; , &quot;#regisPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
	  },500);
	}
	function closeModal(){
	  $(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).hide();
	  $(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;in&quot; , &quot;'&quot; , &quot;);
  	  $(&quot; , &quot;'&quot; , &quot;.modal-backdrop.in&quot; , &quot;'&quot; , &quot;).remove();
	  $(&quot; , &quot;'&quot; , &quot;#sliding-menu&quot; , &quot;'&quot; , &quot;).hide();
	  $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;modal-open&quot; , &quot;'&quot; , &quot;);
	}
	function back(){
	  $(&quot; , &quot;'&quot; , &quot;#forgotPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
	  $(&quot; , &quot;'&quot; , &quot;#regisPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
	  setTimeout(function(){

	  $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;modal-open&quot; , &quot;'&quot; , &quot;);
	  $(&quot; , &quot;'&quot; , &quot;#loginPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
	  },500);
	}


//validate password
  var password = document.getElementById(&quot;member_password&quot;)
  , confirm_password = document.getElementById(&quot;confirm_password&quot;);

  function validatePassword(){
    if(password.value != confirm_password.value) {
      confirm_password.setCustomValidity(&quot;Konfirmasi kata sandi tidak cocok!&quot;);
    } else {
      confirm_password.setCustomValidity(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    }
  }

  password.onchange = validatePassword;
  confirm_password.onkeyup = validatePassword;

  $(document).ready(function(){
    $(&quot;#register_form&quot;).validate({
          rules: {
           member_password: &quot;required&quot;,
           member_first_name: &quot;required&quot;,
           member_birthdate: &quot;required&quot;,
           member_phone: &quot;required&quot;,
           member_email: {
           	required:true,
           	email:true,
           	remote:
           	{
           		url: &quot;https://www.danaeveryday.id/member/register_email_exists&quot;,
           		type: &quot;post&quot;,
           		data:{
           			email: function(){ return $(&quot;.email-regis&quot;).val(); }
           		}
           	}
           },
           &quot;hiddenRecaptcha1&quot;: {
                 required: function() {
                     if(grecaptcha.getResponse() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                         return true;
                     } else {
                         return false;
                     }
                 }
             }
        },
          messages: {
           required: &quot;This Field is Required&quot;,
           member_email: {
           	email: &quot;Email is not valid.&quot;,
           	remote: &quot;Email has been used.&quot;
           },
           &quot;hiddenRecaptcha1&quot;: &quot;Captcha is required&quot;
        }
     });
    $(&quot;#login_form&quot;).validate({
          rules: {
           email: {
           	required:true,
           	email:true
           },
           member_password: &quot;required&quot;
        },
          messages: {
           required: &quot;This Field is Required&quot;,
           email: &quot;Email is not valid&quot;
        }
    });
    $(&quot;#forgot_form&quot;).validate({
          rules: {
           member_email: {
           	required:true,
           	email:true
           }
        },
          messages: {
           required: &quot;This Field is Required&quot;,
           email: &quot;Email is not valid&quot;
        }
    });
  })



	
	
    	
        
        
            
                Bersama Dana Everyday Kita Melestarikan Buddha Sasana di Nusantara &amp; Dunia
                				Yuk, Download!
				
				
                                    
            
			 
                
                    
                
            
        
    

		
			
				
					
						
							210 
							Campaign Terdanai
						
					
					
						
							Rp 39.032.330.317 
							Donasi Tersalurkan
						
					
					
						
							5.507 
							Donatur Tergabung
						
					
				

				
					
						
							
								210 
								Campaign Terdanai
							
						
						
							
								5.507 
								Donatur Tergabung
							
						
					
					
						
							
								Rp 39.032.330.317 
								Donasi Tersalurkan
							
						
					
				

			
		
	

	
	
		
			
				
					
						
							Semua Kategori
															Pembangunan &amp; Renovasi Vihara
															Pendidikan Buddhist
															Sangha Dana
															Sosial Kemanusiaan
															Ashoka Spirit
															Fangshen
							                      	
					
					
						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pembangunan Pagar Vihara Dhammadipa...
									
										Vihara Dhammad?pa loka di bangun pada tahun 1989, pembangunan vihara ini merupakan partisipasi umat Buddha s...									
									
									
										
																						64 hari  sisa waktu
																					
									
									
										4%  tercapai
									
									
										Rp 3.923.101  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Pembangunan PUSDIKLAT Sikkhadama Mahamet...
									
										    PUSDIKLAT Sikkhadama Mahametta Palangkaraya adalah PUSDIKLAT Sangha Theravada Indonesia pertama di P...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										46%  tercapai
									
									
										Rp 459.147.258  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Bangun Kuti Arama Kebun Kesadaran d...
									
										Arama Kebun Kesadaran adalah Salah satu tempat ibadah Agama Buddha di
Provinsi Sulawesi Utara dan merupakan...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										57%  tercapai
									
									
										Rp 276.724.892  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pendidikan Buddhist
									Dana Guru Bahasa &amp; Coding Pesastrian...
									
										Yayasan Kusalamitra sudah membantu anak-anak kurang mampu di berbagai daerah di Indonesia untuk dapat melanj...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										84%  tercapai
									
									
										Rp 69.477.452  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Sangha Dana
									Dana Kendaraan Kepada Sangha untuk Pemba...
									
										Sangha dalam pembabaran Dhamma memerlukan sarana yang dapat menunjang dalam mencapai Vihara-vihara yang jauh...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										34%  tercapai
									
									
										Rp 61.349.392  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana 5 Bidang Relief Buddha di Vihara Sa...
									
										Vihara Sangupati yang dibangun oleh tokoh Buddhis setempat pad tahun 1972 hancur karena gepa Lombok tahun 20...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										31%  tercapai
									
									
										Rp 45.041.458  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pembangunan Lantai Kuti &amp; Pendo...
									
										Ada 3 faktor yang menentukan besarnya jasa kebajikan yang diperoleh dari berdana, yaitu: sifat dari motif pe...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										21%  tercapai
									
									
										Rp 81.828.136  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pembangunan Kuti Vihara Samyag Dars...
									
										Vihara Samyag Darsana kini telah berusia 43 tahun. Ia didirikan dengan semangat
gotong royong oleh umat Bud...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										43%  tercapai
									
									
										Rp 120.361.628  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pemagaran Vihara Giri Ratana Puja L...
									
										Umat Buddha Vihara Giri Ratana Puja Lombok makin menunjukan perkembangan dalam mempelajari dan mempraktekan ...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										77%  tercapai
									
									
										Rp 197.288.160  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pengerjaan Pondasi &amp; Dinding Sa...
									
										Vihara Metta Mandala berdiri sejak tahun 1986 dengan jumlah umat 32 KK (120 Jiwa). Vihara Metta Mandala bera...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										51%  tercapai
									
									
										Rp 84.144.143  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Renovasi Vihara Sangha Metta Arama ...
									
										Vihara Sangha Metta Arama adalah salah satu tempat ibadah umat Buddha aliran Theravada yang bertempat di
Jl...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										84%  tercapai
									
									
										Rp 124.651.099  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pengerjaan Lantai Vihara Vidya Loka...
									
										Vihara Vidya Loka terletak di Dusun Ngroto RT RW 008 005 Desa Sumogawe Kecamatan Getasan Kabupaten Semarang ...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										40%  tercapai
									
									
										Rp 71.183.982  terkumpul
									
												
							
						

											
					Load More
					
						$(document).ready(function() {
							var total_campaigns = 18;
							$(&quot; , &quot;'&quot; , &quot;#campaign-load-more&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(event) {
								event.preventDefault();
								
								$.ajax({
						            url: &quot; , &quot;'&quot; , &quot;https://www.danaeveryday.id/campaign_category/getCampaignFromCategory&quot; , &quot;'&quot; , &quot;,
						            dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
						            data: {
						                    category_id: $(&quot; , &quot;'&quot; , &quot;.category-list.current a&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;category&quot; , &quot;'&quot; , &quot;),
						                    offset: $(&quot; , &quot;'&quot; , &quot;#campaign-load-more&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-offset&quot; , &quot;'&quot; , &quot;)
						                  },
						            beforeSend: function() {
						            	$(&quot; , &quot;'&quot; , &quot;.campaign-crit&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;abs-loading&quot;>&lt;div class=&quot;spinner-loading&quot;>&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
						            },
						            complete: function(json) {
						            	json = json.responseJSON;

						            	// clear campaign list
						            	// $(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
						            	$(&quot; , &quot;'&quot; , &quot;#campaign-load-more&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-offset&quot; , &quot;'&quot; , &quot;, parseInt($(&quot; , &quot;'&quot; , &quot;#campaign-load-more&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-offset&quot; , &quot;'&quot; , &quot;)) + 3);
						            	if (typeof json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
							            	// append campaign
							            	for (var i = 0; i &lt; json.length; i++) {
							            		var donation_collected = json[i][&quot;donation_collected&quot;] ? json[i][&quot;donation_collected&quot;] : 0,
							            			percent = json[i][&quot;percent&quot;],
							            			html = &quot; , &quot;'&quot; , &quot;\
							            				&lt;div class=&quot;col inline l-4 m-6 s-6 min-10 item-sponsor&quot;>\
														&lt;a href=&quot;https://www.danaeveryday.id/campaign/&quot; , &quot;'&quot; , &quot;+ json[i][&quot;seo_url&quot;] +&quot; , &quot;'&quot; , &quot;&quot;>\
															&lt;div class=&quot;hvr-sponsor&quot;>\
																&lt;div class=&quot;img-sponsor&quot;>\
																	&lt;img src=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image&quot;] +&quot; , &quot;'&quot; , &quot;&quot; alt=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image_alt&quot;] +&quot; , &quot;'&quot; , &quot;&quot;/>\
																&lt;/div>\
															&lt;/div>\
															&lt;div class=&quot;pad-sponsor&quot;>\
																&lt;div class=&quot;tag-sponsor&quot;>&lt;i class=&quot;fa fa-tags&quot; aria-hidden=&quot;true&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;categories_name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
																&lt;div class=&quot;n-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
																&lt;div class=&quot;bdy-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ limitString(json[i][&quot;description&quot;]) +&quot; , &quot;'&quot; , &quot;&lt;/div>\
																&lt;div class=&quot;bar-sponsor&quot;>&lt;div class=&quot;in-bar-sponsor&quot; style=&quot;width: &quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;%;&quot;>&lt;/div>&lt;/div>\
																&lt;div>\
																	&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ diffDate(&quot; , &quot;'&quot; , &quot;2024-11-28&quot; , &quot;'&quot; , &quot;, json[i][&quot;end_date&quot;]) +&quot; , &quot;'&quot; , &quot; hari &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>\
																&lt;/div>\
																&lt;div>\
																	&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;% &lt;span class=&quot;o-sponsor&quot;> tercapai&lt;/span>&lt;/div>\
																&lt;/div>\
																&lt;div>\
																	&lt;div class=&quot;d-sponsor&quot;>Rp &quot; , &quot;'&quot; , &quot;+ thousandFormat(donation_collected) +&quot; , &quot;'&quot; , &quot; &lt;span class=&quot;o-sponsor&quot;> terkumpul&lt;/span>&lt;/div>\
																&lt;/div>\
															&lt;/div>\
														&lt;/a>\
													&lt;/div>\
							            			&quot; , &quot;'&quot; , &quot;;

							            		// add campaign
							            		$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).append(html);

							            		// scroll to
							            		var $container = $(&quot;html,body&quot;);
												var $scrollTo = $(&quot; , &quot;'&quot; , &quot;.campaign-crit&quot; , &quot;'&quot; , &quot;);

												$container.animate({scrollTop: $scrollTo.offset().top, scrollLeft: 0},300);
							            	}
							            }
							            else {
							            }
							            	if ($(&quot; , &quot;'&quot; , &quot;.campaign-list .item-sponsor&quot; , &quot;'&quot; , &quot;).length >= total_campaigns) {
							            		$(&quot; , &quot;'&quot; , &quot;#campaign-load-more&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
							            	}

						        		// remove loading
						        		setTimeout(function() {
						        			$(&quot; , &quot;'&quot; , &quot;.abs-loading&quot; , &quot;'&quot; , &quot;).remove();
						        		}, 500);
						            },
						        });
							});
						});
					
				
				
					
						Semua Kategori
													Pembangunan &amp; Renovasi Vihara
													Pendidikan Buddhist
													Sangha Dana
													Sosial Kemanusiaan
													Ashoka Spirit
													Fangshen
											
				
			
		
	
	
		
			
				
					
						
					
				
				
					Dana Everyday mempertemukan ladang berdana yang subur kepada para donatur, seperti: 
					
						
							Dana pembangunan vihara-vihara desa.
							Dana cinta kasih pengobatan umat Buddha yang kurang mampu.
							Baksos untuk umat Buddha yang kurang mampu.
							Dana makan, jubah, keperluan Sangha.
							Visudhi Tisarana.
							Dan masih banyak lagi.
						
					
				
			
		
	
				
		
			
				
				Latest Campaign
				Pilih dan salurkan donasi untuk campaign yang berarti bagi Anda.
				
					
					
						
							
								
									
								
							
							
								Dana Pembangunan Pagar Vihara Dhammadipa...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 3.923.101
										
									
									
										
											Tercapai
											4%
										
									
									
										
											Sisa Waktu
											
																									64 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Pembangunan PUSDIKLAT Sikkhadama Mahamet...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 459.147.258
										
									
									
										
											Tercapai
											46%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Bangun Kuti Arama Kebun Kesadaran d...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 276.724.892
										
									
									
										
											Tercapai
											57%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Guru Bahasa &amp; Coding Pesastrian...
								
									
									Pendidikan Buddhist								
								
								
									
										
											Terkumpul
											Rp 69.477.452
										
									
									
										
											Tercapai
											84%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Kendaraan Kepada Sangha untuk Pemba...
								
									
									Sangha Dana								
								
								
									
										
											Terkumpul
											Rp 61.349.392
										
									
									
										
											Tercapai
											34%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana 5 Bidang Relief Buddha di Vihara Sa...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 45.041.458
										
									
									
										
											Tercapai
											31%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pembangunan Lantai Kuti &amp; Pendo...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 81.828.136
										
									
									
										
											Tercapai
											21%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pembangunan Kuti Vihara Samyag Dars...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 120.361.628
										
									
									
										
											Tercapai
											43%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pemagaran Vihara Giri Ratana Puja L...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 197.288.160
										
									
									
										
											Tercapai
											77%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pengerjaan Pondasi &amp; Dinding Sa...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 84.144.143
										
									
									
										
											Tercapai
											51%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Renovasi Vihara Sangha Metta Arama ...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 124.651.099
										
									
									
										
											Tercapai
											84%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pengerjaan Lantai Vihara Vidya Loka...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 71.183.982
										
									
									
										
											Tercapai
											40%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

										
				
				See All Projects
			
		
	
	
    	
    	    Partners
    	    
    	        
    	            
    	                
    	                
    	       
    	        
    	            MLIKI is a limitless digital item and physical goods marketplace. MLIKI aims to create a hybrid marketplace that makes it possible to purchase physical goods and digital item like NFTs at the touch of a finger
                    
                    
                    
    	        
    	    
    	
	



$(&quot; , &quot;'&quot; , &quot;.slider-banner&quot; , &quot;'&quot; , &quot;).slick({
	autoplay: true,
	autoplaySpeed: 2000,
	arrows: true,
	dots: false,
	slidesToShow: 1,
	slidesToScroll: 1,
});

$(&quot; , &quot;'&quot; , &quot;.slider-campaign&quot; , &quot;'&quot; , &quot;).slick({
	// autoplay: true,
	// autoplaySpeed: 2000,
	// arrows: true,
	// dots: false,
	// slidesToShow: 1,
	// slidesToScroll: 1,
	// centerMode: true
	prevArrow: &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-caret-left campaign-control tY l40-md l20-xs&quot; aria-hidden=&quot;true&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
    nextArrow: &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-caret-right campaign-control tY r40-md r20-xs&quot; aria-hidden=&quot;true&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
    responsive: [
      {
        breakpoint: 767,
        settings: {
    		adaptiveHeight: true
        }
      }
    ]
});

$(&quot;.slider-banner&quot;).on(&quot;afterChange&quot;, function (event, slick, currentSlide, nextSlide){
	var seo = &quot; , &quot;'&quot; , &quot;https://www.danaeveryday.id/campaign/&quot; , &quot;'&quot; , &quot; + $(this).find(&quot; , &quot;'&quot; , &quot;.item[data-slick-index=&quot;&quot; , &quot;'&quot; , &quot;+ currentSlide +&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;seo&quot; , &quot;'&quot; , &quot;);

	$(&quot; , &quot;'&quot; , &quot;.featured-link&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, seo);
})

// $(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li&quot; , &quot;'&quot; , &quot;).click(function(){
// 	var tab_id = $(this).attr(&quot; , &quot;'&quot; , &quot;data-tab&quot; , &quot;'&quot; , &quot;);

// 	$(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// 	$(&quot; , &quot;'&quot; , &quot;.tab-content&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);

// 	$(this).addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// 	$(&quot;.&quot;+tab_id).addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// });

// $(document).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.tab-select&quot; , &quot;'&quot; , &quot;, function(){
// 	console.log($(this).find(&quot; , &quot;'&quot; , &quot;option:selected&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;tab&quot; , &quot;'&quot; , &quot;));
// 	var tab_id = $(this).find(&quot; , &quot;'&quot; , &quot;option:selected&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;tab&quot; , &quot;'&quot; , &quot;);

// 	$(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// 	$(&quot; , &quot;'&quot; , &quot;.tab-content&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);

// 	if (tab_id != &quot; , &quot;'&quot; , &quot;all&quot; , &quot;'&quot; , &quot;) {
// 		$(&quot;.tab-&quot;+tab_id).addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// 	}
// 	else {
// 		$(&quot; , &quot;'&quot; , &quot;.tab-content&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// 	}
// });

$(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li.tab-all&quot; , &quot;'&quot; , &quot;).click(function(){
	$(&quot; , &quot;'&quot; , &quot;.tab-content&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
});

function check_sticky() {
	$(&quot;[data-sticky_column]&quot;).stick_in_parent({
		offset_top: 85,
 		parent: &quot;[data-sticky_parent]&quot;
   	}).on(&quot;sticky_kit:stick&quot;, function(e) {
   		$(&quot; , &quot;'&quot; , &quot;.sticky&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;stuck&quot; , &quot;'&quot; , &quot;);
	}).on(&quot;sticky_kit:unstick&quot;, function(e) {
   		$(&quot; , &quot;'&quot; , &quot;.sticky&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;stuck&quot; , &quot;'&quot; , &quot;);
  	});
}
$(document).ready(function() {
	check_sticky();
});

$(document).ready(function() {
	// ajax categories
	$(document).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.category-list a&quot; , &quot;'&quot; , &quot;, function(event) {
		event.preventDefault();
		var $this = $(this);

		$(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
		$this.parent().addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);

		$.ajax({
            url: &quot; , &quot;'&quot; , &quot;https://www.danaeveryday.id/campaign_category/getCampaignFromCategory&quot; , &quot;'&quot; , &quot;,
            dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
            data: {
                    category_id: $(this).data(&quot; , &quot;'&quot; , &quot;category&quot; , &quot;'&quot; , &quot;)
                  },
            beforeSend: function() {
            	$(&quot; , &quot;'&quot; , &quot;.campaign-crit&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;abs-loading&quot;>&lt;div class=&quot;spinner-loading&quot;>&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);

            	// dont scroll
            	// $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on({
				//     &quot; , &quot;'&quot; , &quot;mousewheel&quot; , &quot;'&quot; , &quot;: function(e) {
				//         e.preventDefault();
				//     }
				// });
            },
            complete: function(json) {
            	json = json.responseJSON;

            	// clear campaign list
            	$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);

            	if (typeof json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
	            	// append campaign
	            	for (var i = 0; i &lt; json.length; i++) {
	            		var donation_collected = json[i][&quot;donation_collected&quot;] ? json[i][&quot;donation_collected&quot;] : 0,
	            			percent = json[i][&quot;percent&quot;],
	            			html = &quot; , &quot;'&quot; , &quot;\
	            				&lt;div class=&quot;col inline l-4 m-6 s-6 min-10 item-sponsor&quot;>\
								&lt;a href=&quot;https://www.danaeveryday.id/campaign/&quot; , &quot;'&quot; , &quot;+ json[i][&quot;seo_url&quot;] +&quot; , &quot;'&quot; , &quot;&quot;>\
									&lt;div class=&quot;hvr-sponsor&quot;>\
										&lt;div class=&quot;img-sponsor&quot;>\
											&lt;img src=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image&quot;] +&quot; , &quot;'&quot; , &quot;&quot; alt=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image_alt&quot;] +&quot; , &quot;'&quot; , &quot;&quot;/>\
										&lt;/div>\
									&lt;/div>\
									&lt;div class=&quot;pad-sponsor&quot;>\
										&lt;div class=&quot;tag-sponsor&quot;>&lt;i class=&quot;fa fa-tags&quot; aria-hidden=&quot;true&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;categories_name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;n-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;bdy-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ limitString(json[i][&quot;description&quot;]) +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;bar-sponsor&quot;>&lt;div class=&quot;in-bar-sponsor&quot; style=&quot;width: &quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;%;&quot;>&lt;/div>&lt;/div>\
										&lt;div>&quot; , &quot;'&quot; , &quot;;
							if (json[i][&quot; , &quot;'&quot; , &quot;date_type&quot; , &quot;'&quot; , &quot;] == 2) {
								html += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;d-sponsor&quot;>&lt;img src=&quot;&quot; , &quot;'&quot; , &quot; + base_url + &quot; , &quot;'&quot; , &quot;lib/images/icons/unlimited.png&quot; style=&quot;width:20px;&quot;> &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>&quot; , &quot;'&quot; , &quot;;
							} else {
								html += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ diffDate(&quot; , &quot;'&quot; , &quot;2024-11-28&quot; , &quot;'&quot; , &quot;, json[i][&quot;end_date&quot;]) +&quot; , &quot;'&quot; , &quot; hari &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>&quot; , &quot;'&quot; , &quot;;
							}
							html +=		&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;% &lt;span class=&quot;o-sponsor&quot;> tercapai&lt;/span>&lt;/div>\
										&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>Rp &quot; , &quot;'&quot; , &quot;+ thousandFormat(donation_collected) +&quot; , &quot;'&quot; , &quot; &lt;span class=&quot;o-sponsor&quot;> terkumpul&lt;/span>&lt;/div>\
										&lt;/div>\
									&lt;/div>\
								&lt;/a>\
							&lt;/div>\
	            			&quot; , &quot;'&quot; , &quot;;

	            		// add campaign
	            		$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).append(html);

	            		// scroll to
	            		var $container = $(&quot;html,body&quot;);
						var $scrollTo = $(&quot; , &quot;'&quot; , &quot;.campaign-crit&quot; , &quot;'&quot; , &quot;);

						$container.animate({scrollTop: $scrollTo.offset().top, scrollLeft: 0},300);
	            	}
	            }
	            else {
	            	$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;col inline&quot;>&lt;div>Campaign Belum Tersedia&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
	            }

        		// remove loading
        		setTimeout(function() {
        			// unbind scroll
        			// $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on({
					//     &quot; , &quot;'&quot; , &quot;mousewheel&quot; , &quot;'&quot; , &quot;: function(e) {
					//         $(this).unbind();
					//     }
					// });

        			$(&quot; , &quot;'&quot; , &quot;.abs-loading&quot; , &quot;'&quot; , &quot;).remove();
        		}, 500);
            },
        });
	});

	$(document).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.tab-select&quot; , &quot;'&quot; , &quot;, function(event) {
		event.preventDefault();
		var cat = $(this).val(),
			$this = $(&quot; , &quot;'&quot; , &quot;a[data-category=&quot; , &quot;'&quot; , &quot;+ cat +&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;);

		$(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
		$this.parent().addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);

		$.ajax({
            url: &quot; , &quot;'&quot; , &quot;https://www.danaeveryday.id/campaign_category/getCampaignFromCategory&quot; , &quot;'&quot; , &quot;,
            dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
            data: {
                    category_id: $(this).val()
                  },
            beforeSend: function() {
            	$(&quot; , &quot;'&quot; , &quot;.campaign-crit&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;abs-loading&quot;>&lt;div class=&quot;spinner-loading&quot;>&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);

            	// dont scroll
            	// $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on({
				//     &quot; , &quot;'&quot; , &quot;mousewheel&quot; , &quot;'&quot; , &quot;: function(e) {
				//         e.preventDefault();
				//     }
				// });
            },
            complete: function(json) {
            	json = json.responseJSON;
				
            	// clear campaign list
            	$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);

            	if (typeof json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
	            	// append campaign
	            	for (var i = 0; i &lt; json.length; i++) {
	            		var percent = Math.ceil((json[i][&quot;donation_collected&quot;]/json[i][&quot;donation&quot;]) * 100),
	            			html = &quot; , &quot;'&quot; , &quot;\
	            				&lt;div class=&quot;col inline l-4 m-6 s-6 min-10 item-sponsor&quot;>\
								&lt;a href=&quot;https://www.danaeveryday.id/campaign/&quot; , &quot;'&quot; , &quot;+ json[i][&quot;seo_url&quot;] +&quot; , &quot;'&quot; , &quot;&quot;>\
									&lt;div class=&quot;hvr-sponsor&quot;>\
										&lt;div class=&quot;img-sponsor&quot;>\
											&lt;img src=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image&quot;] +&quot; , &quot;'&quot; , &quot;&quot; alt=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image_alt&quot;] +&quot; , &quot;'&quot; , &quot;&quot;/>\
										&lt;/div>\
									&lt;/div>\
									&lt;div class=&quot;pad-sponsor&quot;>\
										&lt;div class=&quot;tag-sponsor&quot;>&lt;i class=&quot;fa fa-tags&quot; aria-hidden=&quot;true&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;categories_name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;n-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;bdy-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ limitString(json[i][&quot;description&quot;]) +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;bar-sponsor&quot;>&lt;div class=&quot;in-bar-sponsor&quot; style=&quot;width: &quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;%;&quot;>&lt;/div>&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ diffDate(&quot; , &quot;'&quot; , &quot;2024-11-28&quot; , &quot;'&quot; , &quot;, json[i][&quot;end_date&quot;]) +&quot; , &quot;'&quot; , &quot; hari &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>\
										&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;% &lt;span class=&quot;o-sponsor&quot;> tercapai&lt;/span>&lt;/div>\
										&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>Rp &quot; , &quot;'&quot; , &quot;+ thousandFormat(json[i][&quot;donation_collected&quot;]) +&quot; , &quot;'&quot; , &quot; &lt;span class=&quot;o-sponsor&quot;> terkumpul&lt;/span>&lt;/div>\
										&lt;/div>\
									&lt;/div>\
								&lt;/a>\
							&lt;/div>\
	            			&quot; , &quot;'&quot; , &quot;;

	            		// add campaign
	            		$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).append(html);
	            	}
	            }
	            else {
	            	$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;col inline&quot;>&lt;div>Campaign Belum Tersedia&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
	            }

        		// remove loading
        		setTimeout(function() {
        			// unbind scroll
        			// $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on({
					//     &quot; , &quot;'&quot; , &quot;mousewheel&quot; , &quot;'&quot; , &quot;: function(e) {
					//         $(this).unbind();
					//     }
					// });

        			$(&quot; , &quot;'&quot; , &quot;.abs-loading&quot; , &quot;'&quot; , &quot;).remove();
        		}, 500);
            },
        });
	});
});


	
		
			
				
					
						
							
							Home
						
					
				
				
					
						
							
							Donasi
						
					
				
				
					
												
													
							
															Login
														
						
					
				
				
					
						
							
							Menu
						
					
				
			
		
	
	
		
			
				
					
						
					
				
				
					
						
							TAKE ACTION
							
								Seluruh Campaign
															Galang Dana
														
						
						
							LEARN MORE
							
								Apa itu Dana Everyday?
								Hubungi Kami
								Syarat Dan Ketentuan
							
						
						
							CONNECT
														
								
							
														
								
							
													
					
					
						Copyright 2024 Yayasan Dana Everyday (DEV). All Rights Reserved.
						
						    Gositus
						
					
				
			
		
	



var title = &quot;Home&quot;;
var url = &quot;&quot;;
var url2 = &quot;&quot;;
var recaptcha3;
var recaptcha1;
var recaptcha2;
var recaptcha4;
var recaptcha5;
var recaptcha6;
var recaptcha7;

window.verifyCallback3 = function(response) {
    // return response;
	$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha3&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
	console.log(response)
};
window.verifyCallback1 = function(response) {
	// return response;
	$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha1&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
	console.log(response)
};
if(title == &quot; , &quot;'&quot; , &quot;Reset Password&quot; , &quot;'&quot; , &quot;){
	window.verifyCallback4 = function(response) {
		// return response;
		$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha4&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
		console.log(response)
	};
}
if(title == &quot; , &quot;'&quot; , &quot;Contact Us&quot; , &quot;'&quot; , &quot;){
	window.verifyCallback5 = function(response) {
		// return response;
		$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha5&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
		console.log(response)
	};
}
if(title == &quot; , &quot;'&quot; , &quot;Edit Account&quot; , &quot;'&quot; , &quot;){
	window.verifyCallback6 = function(response) {
		// return response;
		$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha6&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
		console.log(response)
	};
}
if(url == &quot; , &quot;'&quot; , &quot;campaign&quot; , &quot;'&quot; , &quot; &amp;&amp; url2 != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	window.verifyCallback7 = function(response) {
		// return response;
		$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha7&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
		console.log(response)
	};
}
var myCallBack = function() {
    recaptcha1 = grecaptcha.render(&quot; , &quot;'&quot; , &quot;recaptcha1&quot; , &quot;'&quot; , &quot;, {
      &quot; , &quot;'&quot; , &quot;sitekey&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;light&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;callback&quot; , &quot;'&quot; , &quot; : verifyCallback1
    });
    if(title == &quot; , &quot;'&quot; , &quot;Reset Password&quot; , &quot;'&quot; , &quot;){
	    /*recaptcha4 = grecaptcha.render(&quot; , &quot;'&quot; , &quot;recaptcha4&quot; , &quot;'&quot; , &quot;, {
	      &quot; , &quot;'&quot; , &quot;sitekey&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q&quot; , &quot;'&quot; , &quot;, 
	      &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;light&quot; , &quot;'&quot; , &quot;,
	      &quot; , &quot;'&quot; , &quot;callback&quot; , &quot;'&quot; , &quot; : verifyCallback4
	    });*/
	}	
    if(title == &quot; , &quot;'&quot; , &quot;Contact Us&quot; , &quot;'&quot; , &quot;){
	    recaptcha5 = grecaptcha.render(&quot; , &quot;'&quot; , &quot;recaptcha5&quot; , &quot;'&quot; , &quot;, {
	      &quot; , &quot;'&quot; , &quot;sitekey&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q&quot; , &quot;'&quot; , &quot;, 
	      &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;light&quot; , &quot;'&quot; , &quot;,
	      &quot; , &quot;'&quot; , &quot;callback&quot; , &quot;'&quot; , &quot; : verifyCallback5
	    });
	}
    if(title == &quot; , &quot;'&quot; , &quot;Edit Account&quot; , &quot;'&quot; , &quot;){
	    /*recaptcha6 = grecaptcha.render(&quot; , &quot;'&quot; , &quot;recaptcha6&quot; , &quot;'&quot; , &quot;, {
	      &quot; , &quot;'&quot; , &quot;sitekey&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q&quot; , &quot;'&quot; , &quot;, 
	      &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;light&quot; , &quot;'&quot; , &quot;,
	      &quot; , &quot;'&quot; , &quot;callback&quot; , &quot;'&quot; , &quot; : verifyCallback6
	    });*/
	}

    if(url == &quot; , &quot;'&quot; , &quot;campaign&quot; , &quot;'&quot; , &quot; &amp;&amp; url2 != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	    recaptcha7 = grecaptcha.render(&quot; , &quot;'&quot; , &quot;recaptcha7&quot; , &quot;'&quot; , &quot;, {
	      &quot; , &quot;'&quot; , &quot;sitekey&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q&quot; , &quot;'&quot; , &quot;, 
	      &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;light&quot; , &quot;'&quot; , &quot;,
	      &quot; , &quot;'&quot; , &quot;callback&quot; , &quot;'&quot; , &quot; : verifyCallback7
	    });
	}
};

        


  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag(&quot; , &quot;'&quot; , &quot;js&quot; , &quot;'&quot; , &quot;, new Date());

  gtag(&quot; , &quot;'&quot; , &quot;config&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;G-9TQY6RNL58&quot; , &quot;'&quot; , &quot;);


id(&quot;email&quot;)&quot;) or . = concat(&quot;

	
		
			
				
					
						
							
								
									
									
									
										
									
									Donasi
								
							
						
						
															
									
										
											
											
										
										Galang Dana
									
								
													
						
					
				
			
			
				
					
						
					
				
			
			
				
					
						
							Galang Dana
						
					
				
				
					
												
															
									
										
											
											
										
										Login
									
								
								
									
										
											
											
										
										Galang Dana
									
								
													
						
							
								
									
									
									
										
									
									Donasi
								
							
						
					
				
				
					
						
							
								
								
									
										
									
								
							
						
					
								
			
		
	



	
		
			Beranda
		
		
			Tentang Dana Everyday
		
		
			Semua Campaign
		
		
			Kategori Campaign
		
		
			Hubungi Kami
		
		
			Syarat Dan Ketentuan
		
		
		
							Galang Dana
					
		
					
		
					
			




	
	  
	    
	    
	      
	        
	        
		        Selamat Datang!
		        Mohon masukkan email Anda
		    
	      
	      
	      		      		      		      	
	      		
	      			
		              	
		                	
		                		Email
		                		
		                	
		                
		            
	      		
	      		
	      			
		              	
		                	
		                		Password
		                		
		                	
		                
		            
	      		
				
					
						
							
								Lupa password
							
							|
							
								Daftar akun
								
							
												
					
					
			            
			                LOGIN
			            
			        
				
	        
	      
	    

	  
	


	
	  
	    
	    
	      
	      	
	        
	        
		        Selamat Datang!
		        Mohon masukkan email Anda
		    
	      
	      
	      		      	
	      		
	      			
		              	
		                	
		                		Email
		                		
		                	
		                
		            
		        
				
					
			            
			            	
			                SUBMIT
			            
			        
				
	        
	      
	    

	  
	



  
    
    
      
      	
        
        
	        Selamat Datang!
	        Mohon mengisi formulir pendaftaran
	    
      
      
      	      	
      		
      			
	              	
	                	
	                		First Name
	                		
	                	
	                
	            
	            
	            	
	                	
	                		Last Name
	                		
	                	
	                
	            
	        
	        
      			
      				Gender
	              	
                		
                			
                			Laki-Laki
                		
                		
	                		
		                	Perempuan
		                
	                
	            
	        
	        
      			
	              	
	                	
	                		Phone
	                		
	                	
	                
	            
	            
	              	
	                	
	                		Email
	                		
	                	
	                
	            
	        
	        
      			
	              	
	                	
	                		Date Of Birth
	                		
	                	
	                
	            
	        
	        
      			
	              	
	                	
	                		Password
	                		
	                	
	                
	            
	            
	              	
	                	
	                		Confirm Password
	                		
	                	
	                
	            
	        
	        
	        	
					
				      	
				            
			              	
				        
				    
				
				
			   		REGISTER
			   	
			
			
				
					Sudah punya akun? Click disini untuk login
				
			
        
      
    

  


				$(document).ready(function(){
		$(&quot; , &quot;'&quot; , &quot;.angka&quot; , &quot;'&quot; , &quot;).keyup(function () {  
	  		this.value = this.value.replace(/[^0-9.]/g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		});
		$(&quot; , &quot;'&quot; , &quot;.email-val&quot; , &quot;'&quot; , &quot;).keypress(function (event) {
	      var inputValue = event.charCode;

	      //NOTE: in firefox the backspace is keycoded as 0
	        if(!((inputValue > 64 &amp;&amp; inputValue &lt; 91) || (inputValue > 96 &amp;&amp; inputValue &lt; 123)||(inputValue==64)||(inputValue==46)||(inputValue==95)||(inputValue==45)||(inputValue > 47 &amp;&amp; inputValue &lt; 58) || inputValue==0))
	        {
	            event.preventDefault();
	        }
	    });
	    $(&quot; , &quot;'&quot; , &quot;.charNum&quot; , &quot;'&quot; , &quot;).keypress(function (event) {
	      var inputValue = event.charCode;
	        if(!((inputValue > 64 &amp;&amp; inputValue &lt; 91) || (inputValue > 96 &amp;&amp; inputValue &lt; 123)||(inputValue==32)||(inputValue > 47 &amp;&amp; inputValue &lt; 58) || inputValue==0))
	        {
	            event.preventDefault();
	        }
	    });
	    $(&quot; , &quot;'&quot; , &quot;.charOnly&quot; , &quot;'&quot; , &quot;).keypress(function (event) {
	      var inputValue = event.charCode;
	        if(!((inputValue > 64 &amp;&amp; inputValue &lt; 91) || (inputValue > 96 &amp;&amp; inputValue &lt; 123)||(inputValue==32) || inputValue==0))
	        {
	            event.preventDefault();
	        }
	    });
					[].slice.call( document.querySelectorAll( &quot; , &quot;'&quot; , &quot;.form-control&quot; , &quot;'&quot; , &quot;)).forEach( function( inputEl) {
				if( inputEl.value.trim() !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ) {
					
					// $(ev.target.parentNode).addClass(&quot; , &quot;'&quot; , &quot;filled-text&quot; , &quot;'&quot; , &quot; );
				}
				inputEl.addEventListener( &quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, onInputFocus );
				inputEl.addEventListener( &quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;, onInputBlur );
			});

			function onInputFocus( ev ) {
				$(ev.target.parentNode).addClass(&quot; , &quot;'&quot; , &quot;filled-text&quot; , &quot;'&quot; , &quot;);
			}

			function onInputBlur( ev ) {
				if( ev.target.value.trim() === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ) {
					$(ev.target.parentNode).removeClass(&quot; , &quot;'&quot; , &quot;filled-text&quot; , &quot;'&quot; , &quot;);
				}
			}
		
		// $(&quot; , &quot;'&quot; , &quot;#regisPop&quot; , &quot;'&quot; , &quot;).show();
	  	// $(&quot; , &quot;'&quot; , &quot;#regisPop&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;fade&quot; , &quot;'&quot; , &quot;);
	});
	function showlogin(){
	  $(&quot; , &quot;'&quot; , &quot;#loginPop&quot; , &quot;'&quot; , &quot;).show();
	  $(&quot; , &quot;'&quot; , &quot;#loginPop&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;in&quot; , &quot;'&quot; , &quot;);
	  $(&quot; , &quot;'&quot; , &quot;#sliding-menu&quot; , &quot;'&quot; , &quot;).hide();
	  $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;modal-open&quot; , &quot;'&quot; , &quot;);
  	  $(&quot; , &quot;'&quot; , &quot;.modal-backdrop.in&quot; , &quot;'&quot; , &quot;).remove();
	  $(&quot; , &quot;'&quot; , &quot;#regisPop&quot; , &quot;'&quot; , &quot;).hide();
	}
	function showforgot(){
	  $(&quot; , &quot;'&quot; , &quot;#loginPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
	  setTimeout(function(){

	  $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;modal-open&quot; , &quot;'&quot; , &quot;);
	  $(&quot; , &quot;'&quot; , &quot;#forgotPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
	  },500);
	}
	function showregis(){
  	  $(&quot; , &quot;'&quot; , &quot;#loginPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
	  setTimeout(function(){

	  $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;modal-open&quot; , &quot;'&quot; , &quot;);
	  $(&quot; , &quot;'&quot; , &quot;#regisPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
	  },500);
	}
	function closeModal(){
	  $(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).hide();
	  $(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;in&quot; , &quot;'&quot; , &quot;);
  	  $(&quot; , &quot;'&quot; , &quot;.modal-backdrop.in&quot; , &quot;'&quot; , &quot;).remove();
	  $(&quot; , &quot;'&quot; , &quot;#sliding-menu&quot; , &quot;'&quot; , &quot;).hide();
	  $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;modal-open&quot; , &quot;'&quot; , &quot;);
	}
	function back(){
	  $(&quot; , &quot;'&quot; , &quot;#forgotPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
	  $(&quot; , &quot;'&quot; , &quot;#regisPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
	  setTimeout(function(){

	  $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;modal-open&quot; , &quot;'&quot; , &quot;);
	  $(&quot; , &quot;'&quot; , &quot;#loginPop&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
	  },500);
	}


//validate password
  var password = document.getElementById(&quot;member_password&quot;)
  , confirm_password = document.getElementById(&quot;confirm_password&quot;);

  function validatePassword(){
    if(password.value != confirm_password.value) {
      confirm_password.setCustomValidity(&quot;Konfirmasi kata sandi tidak cocok!&quot;);
    } else {
      confirm_password.setCustomValidity(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    }
  }

  password.onchange = validatePassword;
  confirm_password.onkeyup = validatePassword;

  $(document).ready(function(){
    $(&quot;#register_form&quot;).validate({
          rules: {
           member_password: &quot;required&quot;,
           member_first_name: &quot;required&quot;,
           member_birthdate: &quot;required&quot;,
           member_phone: &quot;required&quot;,
           member_email: {
           	required:true,
           	email:true,
           	remote:
           	{
           		url: &quot;https://www.danaeveryday.id/member/register_email_exists&quot;,
           		type: &quot;post&quot;,
           		data:{
           			email: function(){ return $(&quot;.email-regis&quot;).val(); }
           		}
           	}
           },
           &quot;hiddenRecaptcha1&quot;: {
                 required: function() {
                     if(grecaptcha.getResponse() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                         return true;
                     } else {
                         return false;
                     }
                 }
             }
        },
          messages: {
           required: &quot;This Field is Required&quot;,
           member_email: {
           	email: &quot;Email is not valid.&quot;,
           	remote: &quot;Email has been used.&quot;
           },
           &quot;hiddenRecaptcha1&quot;: &quot;Captcha is required&quot;
        }
     });
    $(&quot;#login_form&quot;).validate({
          rules: {
           email: {
           	required:true,
           	email:true
           },
           member_password: &quot;required&quot;
        },
          messages: {
           required: &quot;This Field is Required&quot;,
           email: &quot;Email is not valid&quot;
        }
    });
    $(&quot;#forgot_form&quot;).validate({
          rules: {
           member_email: {
           	required:true,
           	email:true
           }
        },
          messages: {
           required: &quot;This Field is Required&quot;,
           email: &quot;Email is not valid&quot;
        }
    });
  })



	
	
    	
        
        
            
                Bersama Dana Everyday Kita Melestarikan Buddha Sasana di Nusantara &amp; Dunia
                				Yuk, Download!
				
				
                                    
            
			 
                
                    
                
            
        
    

		
			
				
					
						
							210 
							Campaign Terdanai
						
					
					
						
							Rp 39.032.330.317 
							Donasi Tersalurkan
						
					
					
						
							5.507 
							Donatur Tergabung
						
					
				

				
					
						
							
								210 
								Campaign Terdanai
							
						
						
							
								5.507 
								Donatur Tergabung
							
						
					
					
						
							
								Rp 39.032.330.317 
								Donasi Tersalurkan
							
						
					
				

			
		
	

	
	
		
			
				
					
						
							Semua Kategori
															Pembangunan &amp; Renovasi Vihara
															Pendidikan Buddhist
															Sangha Dana
															Sosial Kemanusiaan
															Ashoka Spirit
															Fangshen
							                      	
					
					
						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pembangunan Pagar Vihara Dhammadipa...
									
										Vihara Dhammad?pa loka di bangun pada tahun 1989, pembangunan vihara ini merupakan partisipasi umat Buddha s...									
									
									
										
																						64 hari  sisa waktu
																					
									
									
										4%  tercapai
									
									
										Rp 3.923.101  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Pembangunan PUSDIKLAT Sikkhadama Mahamet...
									
										    PUSDIKLAT Sikkhadama Mahametta Palangkaraya adalah PUSDIKLAT Sangha Theravada Indonesia pertama di P...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										46%  tercapai
									
									
										Rp 459.147.258  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Bangun Kuti Arama Kebun Kesadaran d...
									
										Arama Kebun Kesadaran adalah Salah satu tempat ibadah Agama Buddha di
Provinsi Sulawesi Utara dan merupakan...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										57%  tercapai
									
									
										Rp 276.724.892  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pendidikan Buddhist
									Dana Guru Bahasa &amp; Coding Pesastrian...
									
										Yayasan Kusalamitra sudah membantu anak-anak kurang mampu di berbagai daerah di Indonesia untuk dapat melanj...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										84%  tercapai
									
									
										Rp 69.477.452  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Sangha Dana
									Dana Kendaraan Kepada Sangha untuk Pemba...
									
										Sangha dalam pembabaran Dhamma memerlukan sarana yang dapat menunjang dalam mencapai Vihara-vihara yang jauh...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										34%  tercapai
									
									
										Rp 61.349.392  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana 5 Bidang Relief Buddha di Vihara Sa...
									
										Vihara Sangupati yang dibangun oleh tokoh Buddhis setempat pad tahun 1972 hancur karena gepa Lombok tahun 20...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										31%  tercapai
									
									
										Rp 45.041.458  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pembangunan Lantai Kuti &amp; Pendo...
									
										Ada 3 faktor yang menentukan besarnya jasa kebajikan yang diperoleh dari berdana, yaitu: sifat dari motif pe...									
									
									
										
																						33 hari  sisa waktu
																					
									
									
										21%  tercapai
									
									
										Rp 81.828.136  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pembangunan Kuti Vihara Samyag Dars...
									
										Vihara Samyag Darsana kini telah berusia 43 tahun. Ia didirikan dengan semangat
gotong royong oleh umat Bud...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										43%  tercapai
									
									
										Rp 120.361.628  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pemagaran Vihara Giri Ratana Puja L...
									
										Umat Buddha Vihara Giri Ratana Puja Lombok makin menunjukan perkembangan dalam mempelajari dan mempraktekan ...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										77%  tercapai
									
									
										Rp 197.288.160  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pengerjaan Pondasi &amp; Dinding Sa...
									
										Vihara Metta Mandala berdiri sejak tahun 1986 dengan jumlah umat 32 KK (120 Jiwa). Vihara Metta Mandala bera...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										51%  tercapai
									
									
										Rp 84.144.143  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Renovasi Vihara Sangha Metta Arama ...
									
										Vihara Sangha Metta Arama adalah salah satu tempat ibadah umat Buddha aliran Theravada yang bertempat di
Jl...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										84%  tercapai
									
									
										Rp 124.651.099  terkumpul
									
												
							
						

						
						
							
								
									
										
									
								
								
									Pembangunan &amp; Renovasi Vihara
									Dana Pengerjaan Lantai Vihara Vidya Loka...
									
										Vihara Vidya Loka terletak di Dusun Ngroto RT RW 008 005 Desa Sumogawe Kecamatan Getasan Kabupaten Semarang ...									
									
									
										
																						2 hari  sisa waktu
																					
									
									
										40%  tercapai
									
									
										Rp 71.183.982  terkumpul
									
												
							
						

											
					Load More
					
						$(document).ready(function() {
							var total_campaigns = 18;
							$(&quot; , &quot;'&quot; , &quot;#campaign-load-more&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(event) {
								event.preventDefault();
								
								$.ajax({
						            url: &quot; , &quot;'&quot; , &quot;https://www.danaeveryday.id/campaign_category/getCampaignFromCategory&quot; , &quot;'&quot; , &quot;,
						            dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
						            data: {
						                    category_id: $(&quot; , &quot;'&quot; , &quot;.category-list.current a&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;category&quot; , &quot;'&quot; , &quot;),
						                    offset: $(&quot; , &quot;'&quot; , &quot;#campaign-load-more&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-offset&quot; , &quot;'&quot; , &quot;)
						                  },
						            beforeSend: function() {
						            	$(&quot; , &quot;'&quot; , &quot;.campaign-crit&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;abs-loading&quot;>&lt;div class=&quot;spinner-loading&quot;>&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
						            },
						            complete: function(json) {
						            	json = json.responseJSON;

						            	// clear campaign list
						            	// $(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
						            	$(&quot; , &quot;'&quot; , &quot;#campaign-load-more&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-offset&quot; , &quot;'&quot; , &quot;, parseInt($(&quot; , &quot;'&quot; , &quot;#campaign-load-more&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-offset&quot; , &quot;'&quot; , &quot;)) + 3);
						            	if (typeof json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
							            	// append campaign
							            	for (var i = 0; i &lt; json.length; i++) {
							            		var donation_collected = json[i][&quot;donation_collected&quot;] ? json[i][&quot;donation_collected&quot;] : 0,
							            			percent = json[i][&quot;percent&quot;],
							            			html = &quot; , &quot;'&quot; , &quot;\
							            				&lt;div class=&quot;col inline l-4 m-6 s-6 min-10 item-sponsor&quot;>\
														&lt;a href=&quot;https://www.danaeveryday.id/campaign/&quot; , &quot;'&quot; , &quot;+ json[i][&quot;seo_url&quot;] +&quot; , &quot;'&quot; , &quot;&quot;>\
															&lt;div class=&quot;hvr-sponsor&quot;>\
																&lt;div class=&quot;img-sponsor&quot;>\
																	&lt;img src=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image&quot;] +&quot; , &quot;'&quot; , &quot;&quot; alt=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image_alt&quot;] +&quot; , &quot;'&quot; , &quot;&quot;/>\
																&lt;/div>\
															&lt;/div>\
															&lt;div class=&quot;pad-sponsor&quot;>\
																&lt;div class=&quot;tag-sponsor&quot;>&lt;i class=&quot;fa fa-tags&quot; aria-hidden=&quot;true&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;categories_name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
																&lt;div class=&quot;n-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
																&lt;div class=&quot;bdy-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ limitString(json[i][&quot;description&quot;]) +&quot; , &quot;'&quot; , &quot;&lt;/div>\
																&lt;div class=&quot;bar-sponsor&quot;>&lt;div class=&quot;in-bar-sponsor&quot; style=&quot;width: &quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;%;&quot;>&lt;/div>&lt;/div>\
																&lt;div>\
																	&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ diffDate(&quot; , &quot;'&quot; , &quot;2024-11-28&quot; , &quot;'&quot; , &quot;, json[i][&quot;end_date&quot;]) +&quot; , &quot;'&quot; , &quot; hari &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>\
																&lt;/div>\
																&lt;div>\
																	&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;% &lt;span class=&quot;o-sponsor&quot;> tercapai&lt;/span>&lt;/div>\
																&lt;/div>\
																&lt;div>\
																	&lt;div class=&quot;d-sponsor&quot;>Rp &quot; , &quot;'&quot; , &quot;+ thousandFormat(donation_collected) +&quot; , &quot;'&quot; , &quot; &lt;span class=&quot;o-sponsor&quot;> terkumpul&lt;/span>&lt;/div>\
																&lt;/div>\
															&lt;/div>\
														&lt;/a>\
													&lt;/div>\
							            			&quot; , &quot;'&quot; , &quot;;

							            		// add campaign
							            		$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).append(html);

							            		// scroll to
							            		var $container = $(&quot;html,body&quot;);
												var $scrollTo = $(&quot; , &quot;'&quot; , &quot;.campaign-crit&quot; , &quot;'&quot; , &quot;);

												$container.animate({scrollTop: $scrollTo.offset().top, scrollLeft: 0},300);
							            	}
							            }
							            else {
							            }
							            	if ($(&quot; , &quot;'&quot; , &quot;.campaign-list .item-sponsor&quot; , &quot;'&quot; , &quot;).length >= total_campaigns) {
							            		$(&quot; , &quot;'&quot; , &quot;#campaign-load-more&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
							            	}

						        		// remove loading
						        		setTimeout(function() {
						        			$(&quot; , &quot;'&quot; , &quot;.abs-loading&quot; , &quot;'&quot; , &quot;).remove();
						        		}, 500);
						            },
						        });
							});
						});
					
				
				
					
						Semua Kategori
													Pembangunan &amp; Renovasi Vihara
													Pendidikan Buddhist
													Sangha Dana
													Sosial Kemanusiaan
													Ashoka Spirit
													Fangshen
											
				
			
		
	
	
		
			
				
					
						
					
				
				
					Dana Everyday mempertemukan ladang berdana yang subur kepada para donatur, seperti: 
					
						
							Dana pembangunan vihara-vihara desa.
							Dana cinta kasih pengobatan umat Buddha yang kurang mampu.
							Baksos untuk umat Buddha yang kurang mampu.
							Dana makan, jubah, keperluan Sangha.
							Visudhi Tisarana.
							Dan masih banyak lagi.
						
					
				
			
		
	
				
		
			
				
				Latest Campaign
				Pilih dan salurkan donasi untuk campaign yang berarti bagi Anda.
				
					
					
						
							
								
									
								
							
							
								Dana Pembangunan Pagar Vihara Dhammadipa...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 3.923.101
										
									
									
										
											Tercapai
											4%
										
									
									
										
											Sisa Waktu
											
																									64 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Pembangunan PUSDIKLAT Sikkhadama Mahamet...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 459.147.258
										
									
									
										
											Tercapai
											46%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Bangun Kuti Arama Kebun Kesadaran d...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 276.724.892
										
									
									
										
											Tercapai
											57%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Guru Bahasa &amp; Coding Pesastrian...
								
									
									Pendidikan Buddhist								
								
								
									
										
											Terkumpul
											Rp 69.477.452
										
									
									
										
											Tercapai
											84%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Kendaraan Kepada Sangha untuk Pemba...
								
									
									Sangha Dana								
								
								
									
										
											Terkumpul
											Rp 61.349.392
										
									
									
										
											Tercapai
											34%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana 5 Bidang Relief Buddha di Vihara Sa...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 45.041.458
										
									
									
										
											Tercapai
											31%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pembangunan Lantai Kuti &amp; Pendo...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 81.828.136
										
									
									
										
											Tercapai
											21%
										
									
									
										
											Sisa Waktu
											
																									33 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pembangunan Kuti Vihara Samyag Dars...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 120.361.628
										
									
									
										
											Tercapai
											43%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pemagaran Vihara Giri Ratana Puja L...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 197.288.160
										
									
									
										
											Tercapai
											77%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pengerjaan Pondasi &amp; Dinding Sa...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 84.144.143
										
									
									
										
											Tercapai
											51%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Renovasi Vihara Sangha Metta Arama ...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 124.651.099
										
									
									
										
											Tercapai
											84%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

					
					
						
							
								
									
								
							
							
								Dana Pengerjaan Lantai Vihara Vidya Loka...
								
									
									Pembangunan &amp; Renovasi Vihara								
								
								
									
										
											Terkumpul
											Rp 71.183.982
										
									
									
										
											Tercapai
											40%
										
									
									
										
											Sisa Waktu
											
																									2 hari
																							
										
									
								
											
						
					

										
				
				See All Projects
			
		
	
	
    	
    	    Partners
    	    
    	        
    	            
    	                
    	                
    	       
    	        
    	            MLIKI is a limitless digital item and physical goods marketplace. MLIKI aims to create a hybrid marketplace that makes it possible to purchase physical goods and digital item like NFTs at the touch of a finger
                    
                    
                    
    	        
    	    
    	
	



$(&quot; , &quot;'&quot; , &quot;.slider-banner&quot; , &quot;'&quot; , &quot;).slick({
	autoplay: true,
	autoplaySpeed: 2000,
	arrows: true,
	dots: false,
	slidesToShow: 1,
	slidesToScroll: 1,
});

$(&quot; , &quot;'&quot; , &quot;.slider-campaign&quot; , &quot;'&quot; , &quot;).slick({
	// autoplay: true,
	// autoplaySpeed: 2000,
	// arrows: true,
	// dots: false,
	// slidesToShow: 1,
	// slidesToScroll: 1,
	// centerMode: true
	prevArrow: &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-caret-left campaign-control tY l40-md l20-xs&quot; aria-hidden=&quot;true&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
    nextArrow: &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-caret-right campaign-control tY r40-md r20-xs&quot; aria-hidden=&quot;true&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;,
    responsive: [
      {
        breakpoint: 767,
        settings: {
    		adaptiveHeight: true
        }
      }
    ]
});

$(&quot;.slider-banner&quot;).on(&quot;afterChange&quot;, function (event, slick, currentSlide, nextSlide){
	var seo = &quot; , &quot;'&quot; , &quot;https://www.danaeveryday.id/campaign/&quot; , &quot;'&quot; , &quot; + $(this).find(&quot; , &quot;'&quot; , &quot;.item[data-slick-index=&quot;&quot; , &quot;'&quot; , &quot;+ currentSlide +&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;seo&quot; , &quot;'&quot; , &quot;);

	$(&quot; , &quot;'&quot; , &quot;.featured-link&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, seo);
})

// $(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li&quot; , &quot;'&quot; , &quot;).click(function(){
// 	var tab_id = $(this).attr(&quot; , &quot;'&quot; , &quot;data-tab&quot; , &quot;'&quot; , &quot;);

// 	$(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// 	$(&quot; , &quot;'&quot; , &quot;.tab-content&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);

// 	$(this).addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// 	$(&quot;.&quot;+tab_id).addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// });

// $(document).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.tab-select&quot; , &quot;'&quot; , &quot;, function(){
// 	console.log($(this).find(&quot; , &quot;'&quot; , &quot;option:selected&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;tab&quot; , &quot;'&quot; , &quot;));
// 	var tab_id = $(this).find(&quot; , &quot;'&quot; , &quot;option:selected&quot; , &quot;'&quot; , &quot;).data(&quot; , &quot;'&quot; , &quot;tab&quot; , &quot;'&quot; , &quot;);

// 	$(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// 	$(&quot; , &quot;'&quot; , &quot;.tab-content&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);

// 	if (tab_id != &quot; , &quot;'&quot; , &quot;all&quot; , &quot;'&quot; , &quot;) {
// 		$(&quot;.tab-&quot;+tab_id).addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// 	}
// 	else {
// 		$(&quot; , &quot;'&quot; , &quot;.tab-content&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
// 	}
// });

$(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li.tab-all&quot; , &quot;'&quot; , &quot;).click(function(){
	$(&quot; , &quot;'&quot; , &quot;.tab-content&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
});

function check_sticky() {
	$(&quot;[data-sticky_column]&quot;).stick_in_parent({
		offset_top: 85,
 		parent: &quot;[data-sticky_parent]&quot;
   	}).on(&quot;sticky_kit:stick&quot;, function(e) {
   		$(&quot; , &quot;'&quot; , &quot;.sticky&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;stuck&quot; , &quot;'&quot; , &quot;);
	}).on(&quot;sticky_kit:unstick&quot;, function(e) {
   		$(&quot; , &quot;'&quot; , &quot;.sticky&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;stuck&quot; , &quot;'&quot; , &quot;);
  	});
}
$(document).ready(function() {
	check_sticky();
});

$(document).ready(function() {
	// ajax categories
	$(document).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.category-list a&quot; , &quot;'&quot; , &quot;, function(event) {
		event.preventDefault();
		var $this = $(this);

		$(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
		$this.parent().addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);

		$.ajax({
            url: &quot; , &quot;'&quot; , &quot;https://www.danaeveryday.id/campaign_category/getCampaignFromCategory&quot; , &quot;'&quot; , &quot;,
            dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
            data: {
                    category_id: $(this).data(&quot; , &quot;'&quot; , &quot;category&quot; , &quot;'&quot; , &quot;)
                  },
            beforeSend: function() {
            	$(&quot; , &quot;'&quot; , &quot;.campaign-crit&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;abs-loading&quot;>&lt;div class=&quot;spinner-loading&quot;>&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);

            	// dont scroll
            	// $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on({
				//     &quot; , &quot;'&quot; , &quot;mousewheel&quot; , &quot;'&quot; , &quot;: function(e) {
				//         e.preventDefault();
				//     }
				// });
            },
            complete: function(json) {
            	json = json.responseJSON;

            	// clear campaign list
            	$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);

            	if (typeof json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
	            	// append campaign
	            	for (var i = 0; i &lt; json.length; i++) {
	            		var donation_collected = json[i][&quot;donation_collected&quot;] ? json[i][&quot;donation_collected&quot;] : 0,
	            			percent = json[i][&quot;percent&quot;],
	            			html = &quot; , &quot;'&quot; , &quot;\
	            				&lt;div class=&quot;col inline l-4 m-6 s-6 min-10 item-sponsor&quot;>\
								&lt;a href=&quot;https://www.danaeveryday.id/campaign/&quot; , &quot;'&quot; , &quot;+ json[i][&quot;seo_url&quot;] +&quot; , &quot;'&quot; , &quot;&quot;>\
									&lt;div class=&quot;hvr-sponsor&quot;>\
										&lt;div class=&quot;img-sponsor&quot;>\
											&lt;img src=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image&quot;] +&quot; , &quot;'&quot; , &quot;&quot; alt=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image_alt&quot;] +&quot; , &quot;'&quot; , &quot;&quot;/>\
										&lt;/div>\
									&lt;/div>\
									&lt;div class=&quot;pad-sponsor&quot;>\
										&lt;div class=&quot;tag-sponsor&quot;>&lt;i class=&quot;fa fa-tags&quot; aria-hidden=&quot;true&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;categories_name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;n-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;bdy-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ limitString(json[i][&quot;description&quot;]) +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;bar-sponsor&quot;>&lt;div class=&quot;in-bar-sponsor&quot; style=&quot;width: &quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;%;&quot;>&lt;/div>&lt;/div>\
										&lt;div>&quot; , &quot;'&quot; , &quot;;
							if (json[i][&quot; , &quot;'&quot; , &quot;date_type&quot; , &quot;'&quot; , &quot;] == 2) {
								html += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;d-sponsor&quot;>&lt;img src=&quot;&quot; , &quot;'&quot; , &quot; + base_url + &quot; , &quot;'&quot; , &quot;lib/images/icons/unlimited.png&quot; style=&quot;width:20px;&quot;> &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>&quot; , &quot;'&quot; , &quot;;
							} else {
								html += &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ diffDate(&quot; , &quot;'&quot; , &quot;2024-11-28&quot; , &quot;'&quot; , &quot;, json[i][&quot;end_date&quot;]) +&quot; , &quot;'&quot; , &quot; hari &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>&quot; , &quot;'&quot; , &quot;;
							}
							html +=		&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;% &lt;span class=&quot;o-sponsor&quot;> tercapai&lt;/span>&lt;/div>\
										&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>Rp &quot; , &quot;'&quot; , &quot;+ thousandFormat(donation_collected) +&quot; , &quot;'&quot; , &quot; &lt;span class=&quot;o-sponsor&quot;> terkumpul&lt;/span>&lt;/div>\
										&lt;/div>\
									&lt;/div>\
								&lt;/a>\
							&lt;/div>\
	            			&quot; , &quot;'&quot; , &quot;;

	            		// add campaign
	            		$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).append(html);

	            		// scroll to
	            		var $container = $(&quot;html,body&quot;);
						var $scrollTo = $(&quot; , &quot;'&quot; , &quot;.campaign-crit&quot; , &quot;'&quot; , &quot;);

						$container.animate({scrollTop: $scrollTo.offset().top, scrollLeft: 0},300);
	            	}
	            }
	            else {
	            	$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;col inline&quot;>&lt;div>Campaign Belum Tersedia&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
	            }

        		// remove loading
        		setTimeout(function() {
        			// unbind scroll
        			// $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on({
					//     &quot; , &quot;'&quot; , &quot;mousewheel&quot; , &quot;'&quot; , &quot;: function(e) {
					//         $(this).unbind();
					//     }
					// });

        			$(&quot; , &quot;'&quot; , &quot;.abs-loading&quot; , &quot;'&quot; , &quot;).remove();
        		}, 500);
            },
        });
	});

	$(document).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.tab-select&quot; , &quot;'&quot; , &quot;, function(event) {
		event.preventDefault();
		var cat = $(this).val(),
			$this = $(&quot; , &quot;'&quot; , &quot;a[data-category=&quot; , &quot;'&quot; , &quot;+ cat +&quot; , &quot;'&quot; , &quot;]&quot; , &quot;'&quot; , &quot;);

		$(&quot; , &quot;'&quot; , &quot;ul.l-sponsor li&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);
		$this.parent().addClass(&quot; , &quot;'&quot; , &quot;current&quot; , &quot;'&quot; , &quot;);

		$.ajax({
            url: &quot; , &quot;'&quot; , &quot;https://www.danaeveryday.id/campaign_category/getCampaignFromCategory&quot; , &quot;'&quot; , &quot;,
            dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
            data: {
                    category_id: $(this).val()
                  },
            beforeSend: function() {
            	$(&quot; , &quot;'&quot; , &quot;.campaign-crit&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;abs-loading&quot;>&lt;div class=&quot;spinner-loading&quot;>&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);

            	// dont scroll
            	// $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on({
				//     &quot; , &quot;'&quot; , &quot;mousewheel&quot; , &quot;'&quot; , &quot;: function(e) {
				//         e.preventDefault();
				//     }
				// });
            },
            complete: function(json) {
            	json = json.responseJSON;
				
            	// clear campaign list
            	$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);

            	if (typeof json[&quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;] === &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
	            	// append campaign
	            	for (var i = 0; i &lt; json.length; i++) {
	            		var percent = Math.ceil((json[i][&quot;donation_collected&quot;]/json[i][&quot;donation&quot;]) * 100),
	            			html = &quot; , &quot;'&quot; , &quot;\
	            				&lt;div class=&quot;col inline l-4 m-6 s-6 min-10 item-sponsor&quot;>\
								&lt;a href=&quot;https://www.danaeveryday.id/campaign/&quot; , &quot;'&quot; , &quot;+ json[i][&quot;seo_url&quot;] +&quot; , &quot;'&quot; , &quot;&quot;>\
									&lt;div class=&quot;hvr-sponsor&quot;>\
										&lt;div class=&quot;img-sponsor&quot;>\
											&lt;img src=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image&quot;] +&quot; , &quot;'&quot; , &quot;&quot; alt=&quot;&quot; , &quot;'&quot; , &quot;+ json[i][&quot;image_alt&quot;] +&quot; , &quot;'&quot; , &quot;&quot;/>\
										&lt;/div>\
									&lt;/div>\
									&lt;div class=&quot;pad-sponsor&quot;>\
										&lt;div class=&quot;tag-sponsor&quot;>&lt;i class=&quot;fa fa-tags&quot; aria-hidden=&quot;true&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;categories_name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;n-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ json[i][&quot;name&quot;] +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;bdy-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ limitString(json[i][&quot;description&quot;]) +&quot; , &quot;'&quot; , &quot;&lt;/div>\
										&lt;div class=&quot;bar-sponsor&quot;>&lt;div class=&quot;in-bar-sponsor&quot; style=&quot;width: &quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;%;&quot;>&lt;/div>&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ diffDate(&quot; , &quot;'&quot; , &quot;2024-11-28&quot; , &quot;'&quot; , &quot;, json[i][&quot;end_date&quot;]) +&quot; , &quot;'&quot; , &quot; hari &lt;span class=&quot;o-sponsor&quot;> sisa waktu&lt;/span>&lt;/div>\
										&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>&quot; , &quot;'&quot; , &quot;+ percent +&quot; , &quot;'&quot; , &quot;% &lt;span class=&quot;o-sponsor&quot;> tercapai&lt;/span>&lt;/div>\
										&lt;/div>\
										&lt;div>\
											&lt;div class=&quot;d-sponsor&quot;>Rp &quot; , &quot;'&quot; , &quot;+ thousandFormat(json[i][&quot;donation_collected&quot;]) +&quot; , &quot;'&quot; , &quot; &lt;span class=&quot;o-sponsor&quot;> terkumpul&lt;/span>&lt;/div>\
										&lt;/div>\
									&lt;/div>\
								&lt;/a>\
							&lt;/div>\
	            			&quot; , &quot;'&quot; , &quot;;

	            		// add campaign
	            		$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).append(html);
	            	}
	            }
	            else {
	            	$(&quot; , &quot;'&quot; , &quot;.campaign-list&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;col inline&quot;>&lt;div>Campaign Belum Tersedia&lt;/div>&lt;/div>&quot; , &quot;'&quot; , &quot;);
	            }

        		// remove loading
        		setTimeout(function() {
        			// unbind scroll
        			// $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on({
					//     &quot; , &quot;'&quot; , &quot;mousewheel&quot; , &quot;'&quot; , &quot;: function(e) {
					//         $(this).unbind();
					//     }
					// });

        			$(&quot; , &quot;'&quot; , &quot;.abs-loading&quot; , &quot;'&quot; , &quot;).remove();
        		}, 500);
            },
        });
	});
});


	
		
			
				
					
						
							
							Home
						
					
				
				
					
						
							
							Donasi
						
					
				
				
					
												
													
							
															Login
														
						
					
				
				
					
						
							
							Menu
						
					
				
			
		
	
	
		
			
				
					
						
					
				
				
					
						
							TAKE ACTION
							
								Seluruh Campaign
															Galang Dana
														
						
						
							LEARN MORE
							
								Apa itu Dana Everyday?
								Hubungi Kami
								Syarat Dan Ketentuan
							
						
						
							CONNECT
														
								
							
														
								
							
													
					
					
						Copyright 2024 Yayasan Dana Everyday (DEV). All Rights Reserved.
						
						    Gositus
						
					
				
			
		
	



var title = &quot;Home&quot;;
var url = &quot;&quot;;
var url2 = &quot;&quot;;
var recaptcha3;
var recaptcha1;
var recaptcha2;
var recaptcha4;
var recaptcha5;
var recaptcha6;
var recaptcha7;

window.verifyCallback3 = function(response) {
    // return response;
	$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha3&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
	console.log(response)
};
window.verifyCallback1 = function(response) {
	// return response;
	$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha1&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
	console.log(response)
};
if(title == &quot; , &quot;'&quot; , &quot;Reset Password&quot; , &quot;'&quot; , &quot;){
	window.verifyCallback4 = function(response) {
		// return response;
		$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha4&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
		console.log(response)
	};
}
if(title == &quot; , &quot;'&quot; , &quot;Contact Us&quot; , &quot;'&quot; , &quot;){
	window.verifyCallback5 = function(response) {
		// return response;
		$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha5&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
		console.log(response)
	};
}
if(title == &quot; , &quot;'&quot; , &quot;Edit Account&quot; , &quot;'&quot; , &quot;){
	window.verifyCallback6 = function(response) {
		// return response;
		$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha6&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
		console.log(response)
	};
}
if(url == &quot; , &quot;'&quot; , &quot;campaign&quot; , &quot;'&quot; , &quot; &amp;&amp; url2 != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	window.verifyCallback7 = function(response) {
		// return response;
		$(&quot; , &quot;'&quot; , &quot;#hiddenRecaptcha7&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;value&quot; , &quot;'&quot; , &quot;, response);
		console.log(response)
	};
}
var myCallBack = function() {
    recaptcha1 = grecaptcha.render(&quot; , &quot;'&quot; , &quot;recaptcha1&quot; , &quot;'&quot; , &quot;, {
      &quot; , &quot;'&quot; , &quot;sitekey&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;light&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;callback&quot; , &quot;'&quot; , &quot; : verifyCallback1
    });
    if(title == &quot; , &quot;'&quot; , &quot;Reset Password&quot; , &quot;'&quot; , &quot;){
	    /*recaptcha4 = grecaptcha.render(&quot; , &quot;'&quot; , &quot;recaptcha4&quot; , &quot;'&quot; , &quot;, {
	      &quot; , &quot;'&quot; , &quot;sitekey&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q&quot; , &quot;'&quot; , &quot;, 
	      &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;light&quot; , &quot;'&quot; , &quot;,
	      &quot; , &quot;'&quot; , &quot;callback&quot; , &quot;'&quot; , &quot; : verifyCallback4
	    });*/
	}	
    if(title == &quot; , &quot;'&quot; , &quot;Contact Us&quot; , &quot;'&quot; , &quot;){
	    recaptcha5 = grecaptcha.render(&quot; , &quot;'&quot; , &quot;recaptcha5&quot; , &quot;'&quot; , &quot;, {
	      &quot; , &quot;'&quot; , &quot;sitekey&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q&quot; , &quot;'&quot; , &quot;, 
	      &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;light&quot; , &quot;'&quot; , &quot;,
	      &quot; , &quot;'&quot; , &quot;callback&quot; , &quot;'&quot; , &quot; : verifyCallback5
	    });
	}
    if(title == &quot; , &quot;'&quot; , &quot;Edit Account&quot; , &quot;'&quot; , &quot;){
	    /*recaptcha6 = grecaptcha.render(&quot; , &quot;'&quot; , &quot;recaptcha6&quot; , &quot;'&quot; , &quot;, {
	      &quot; , &quot;'&quot; , &quot;sitekey&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q&quot; , &quot;'&quot; , &quot;, 
	      &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;light&quot; , &quot;'&quot; , &quot;,
	      &quot; , &quot;'&quot; , &quot;callback&quot; , &quot;'&quot; , &quot; : verifyCallback6
	    });*/
	}

    if(url == &quot; , &quot;'&quot; , &quot;campaign&quot; , &quot;'&quot; , &quot; &amp;&amp; url2 != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	    recaptcha7 = grecaptcha.render(&quot; , &quot;'&quot; , &quot;recaptcha7&quot; , &quot;'&quot; , &quot;, {
	      &quot; , &quot;'&quot; , &quot;sitekey&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;6LePsiIUAAAAAFWREQBnbPENrZW7DPile3PwS47q&quot; , &quot;'&quot; , &quot;, 
	      &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;light&quot; , &quot;'&quot; , &quot;,
	      &quot; , &quot;'&quot; , &quot;callback&quot; , &quot;'&quot; , &quot; : verifyCallback7
	    });
	}
};

        


  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag(&quot; , &quot;'&quot; , &quot;js&quot; , &quot;'&quot; , &quot;, new Date());

  gtag(&quot; , &quot;'&quot; , &quot;config&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;G-9TQY6RNL58&quot; , &quot;'&quot; , &quot;);


id(&quot;email&quot;)&quot;))]</value>
      <webElementGuid>d51df3e4-9a93-45df-b640-c9549e2da4ea</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
