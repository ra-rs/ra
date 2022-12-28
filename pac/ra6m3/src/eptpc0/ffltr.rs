#[doc = "Register `FFLTR` reader"]
pub struct R(crate::R<FFLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFLTR` writer"]
pub struct W(crate::W<FFLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FFLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Receive MAC Address SelectNOTE: The setting of these bits is only effective when EXTPRM=0, ENB=1and RPT=1."]
pub type SEL_R = crate::BitReader<SEL_A>;
#[doc = "Receive MAC Address SelectNOTE: The setting of these bits is only effective when EXTPRM=0, ENB=1and RPT=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL_A {
    #[doc = "0: Only receive multicast frames matching the MAC address setting in FMAC0R(U/L)."]
    _0 = 0,
    #[doc = "1: Only receive multicast frames matching the MAC address setting in FMAC1R(U/L)."]
    _1 = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::_0,
            true => SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEL_A::_1
    }
}
#[doc = "Field `SEL` writer - Receive MAC Address SelectNOTE: The setting of these bits is only effective when EXTPRM=0, ENB=1and RPT=1."]
pub type SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFLTR_SPEC, SEL_A, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Only receive multicast frames matching the MAC address setting in FMAC0R(U/L)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEL_A::_0)
    }
    #[doc = "Only receive multicast frames matching the MAC address setting in FMAC1R(U/L)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEL_A::_1)
    }
}
#[doc = "Field `PRT` reader - Frame Reception EnableNOTE: The setting of these bits is only effective when EXTPRM=0 and ENB=1."]
pub type PRT_R = crate::BitReader<PRT_A>;
#[doc = "Frame Reception EnableNOTE: The setting of these bits is only effective when EXTPRM=0 and ENB=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRT_A {
    #[doc = "0: Do not receive multicast frames."]
    _0 = 0,
    #[doc = "1: See SEL bit."]
    _1 = 1,
}
impl From<PRT_A> for bool {
    #[inline(always)]
    fn from(variant: PRT_A) -> Self {
        variant as u8 != 0
    }
}
impl PRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRT_A {
        match self.bits {
            false => PRT_A::_0,
            true => PRT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRT_A::_1
    }
}
#[doc = "Field `PRT` writer - Frame Reception EnableNOTE: The setting of these bits is only effective when EXTPRM=0 and ENB=1."]
pub type PRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFLTR_SPEC, PRT_A, O>;
impl<'a, const O: u8> PRT_W<'a, O> {
    #[doc = "Do not receive multicast frames."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRT_A::_0)
    }
    #[doc = "See SEL bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRT_A::_1)
    }
}
#[doc = "Field `ENB` reader - Reception Filter EnableNOTE: The setting of these bits is only effective when EXTPRM=0."]
pub type ENB_R = crate::BitReader<ENB_A>;
#[doc = "Reception Filter EnableNOTE: The setting of these bits is only effective when EXTPRM=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    #[doc = "0: Filtering is disabled (all multicast frames are received)."]
    _0 = 0,
    #[doc = "1: See PRT and SEL bit."]
    _1 = 1,
}
impl From<ENB_A> for bool {
    #[inline(always)]
    fn from(variant: ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENB_A {
        match self.bits {
            false => ENB_A::_0,
            true => ENB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENB_A::_1
    }
}
#[doc = "Field `ENB` writer - Reception Filter EnableNOTE: The setting of these bits is only effective when EXTPRM=0."]
pub type ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFLTR_SPEC, ENB_A, O>;
impl<'a, const O: u8> ENB_W<'a, O> {
    #[doc = "Filtering is disabled (all multicast frames are received)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENB_A::_0)
    }
    #[doc = "See PRT and SEL bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENB_A::_1)
    }
}
#[doc = "Field `EXTPRM` reader - Extended Promiscuous ModeSetting"]
pub type EXTPRM_R = crate::BitReader<EXTPRM_A>;
#[doc = "Extended Promiscuous ModeSetting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTPRM_A {
    #[doc = "0: Normal operation (unicast frames addressed to the EPTPC are received, filtering of PTP frames is applied, multicast filtering is applied, and all broadcast frames are received)."]
    _0 = 0,
    #[doc = "1: Extended promiscuous mode (all frames are received)"]
    _1 = 1,
}
impl From<EXTPRM_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPRM_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTPRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTPRM_A {
        match self.bits {
            false => EXTPRM_A::_0,
            true => EXTPRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXTPRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXTPRM_A::_1
    }
}
#[doc = "Field `EXTPRM` writer - Extended Promiscuous ModeSetting"]
pub type EXTPRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFLTR_SPEC, EXTPRM_A, O>;
impl<'a, const O: u8> EXTPRM_W<'a, O> {
    #[doc = "Normal operation (unicast frames addressed to the EPTPC are received, filtering of PTP frames is applied, multicast filtering is applied, and all broadcast frames are received)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTPRM_A::_0)
    }
    #[doc = "Extended promiscuous mode (all frames are received)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTPRM_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive MAC Address SelectNOTE: The setting of these bits is only effective when EXTPRM=0, ENB=1and RPT=1."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Reception EnableNOTE: The setting of these bits is only effective when EXTPRM=0 and ENB=1."]
    #[inline(always)]
    pub fn prt(&self) -> PRT_R {
        PRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reception Filter EnableNOTE: The setting of these bits is only effective when EXTPRM=0."]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Extended Promiscuous ModeSetting"]
    #[inline(always)]
    pub fn extprm(&self) -> EXTPRM_R {
        EXTPRM_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive MAC Address SelectNOTE: The setting of these bits is only effective when EXTPRM=0, ENB=1and RPT=1."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Bit 1 - Frame Reception EnableNOTE: The setting of these bits is only effective when EXTPRM=0 and ENB=1."]
    #[inline(always)]
    #[must_use]
    pub fn prt(&mut self) -> PRT_W<1> {
        PRT_W::new(self)
    }
    #[doc = "Bit 2 - Reception Filter EnableNOTE: The setting of these bits is only effective when EXTPRM=0."]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<2> {
        ENB_W::new(self)
    }
    #[doc = "Bit 16 - Extended Promiscuous ModeSetting"]
    #[inline(always)]
    #[must_use]
    pub fn extprm(&mut self) -> EXTPRM_W<16> {
        EXTPRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Reception Filter Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffltr](index.html) module"]
pub struct FFLTR_SPEC;
impl crate::RegisterSpec for FFLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffltr::R](R) reader structure"]
impl crate::Readable for FFLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffltr::W](W) writer structure"]
impl crate::Writable for FFLTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FFLTR to value 0x0001_0000"]
impl crate::Resettable for FFLTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
