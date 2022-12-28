#[doc = "Register `DCDCCTL` reader"]
pub struct R(crate::R<DCDCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCCTL` writer"]
pub struct W(crate::W<DCDCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCCTL_SPEC>;
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
impl From<crate::W<DCDCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDCON` reader - LDO/DCDC on/off Control bit"]
pub type DCDCON_R = crate::BitReader<DCDCON_A>;
#[doc = "LDO/DCDC on/off Control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDCON_A {
    #[doc = "0: LDO is on and DCDC is off"]
    _0 = 0,
    #[doc = "1: LDO is off and DCDC is on"]
    _1 = 1,
}
impl From<DCDCON_A> for bool {
    #[inline(always)]
    fn from(variant: DCDCON_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDCON_A {
        match self.bits {
            false => DCDCON_A::_0,
            true => DCDCON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCDCON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCDCON_A::_1
    }
}
#[doc = "Field `DCDCON` writer - LDO/DCDC on/off Control bit"]
pub type DCDCON_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCDCCTL_SPEC, DCDCON_A, O>;
impl<'a, const O: u8> DCDCON_W<'a, O> {
    #[doc = "LDO is on and DCDC is off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCDCON_A::_0)
    }
    #[doc = "LDO is off and DCDC is on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCDCON_A::_1)
    }
}
#[doc = "Field `OCPEN` reader - DCDC OCP Function Enable bit"]
pub type OCPEN_R = crate::BitReader<OCPEN_A>;
#[doc = "DCDC OCP Function Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCPEN_A {
    #[doc = "0: DCDC OCP (Over Current Protection) Function disable"]
    _0 = 0,
    #[doc = "1: DCDC OCP (Over Current Protection) Function enable"]
    _1 = 1,
}
impl From<OCPEN_A> for bool {
    #[inline(always)]
    fn from(variant: OCPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OCPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCPEN_A {
        match self.bits {
            false => OCPEN_A::_0,
            true => OCPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OCPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OCPEN_A::_1
    }
}
#[doc = "Field `OCPEN` writer - DCDC OCP Function Enable bit"]
pub type OCPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCDCCTL_SPEC, OCPEN_A, O>;
impl<'a, const O: u8> OCPEN_W<'a, O> {
    #[doc = "DCDC OCP (Over Current Protection) Function disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCPEN_A::_0)
    }
    #[doc = "DCDC OCP (Over Current Protection) Function enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCPEN_A::_1)
    }
}
#[doc = "Field `STOPZA` reader - DCDC IO Buffer Power Control bit"]
pub type STOPZA_R = crate::BitReader<STOPZA_A>;
#[doc = "DCDC IO Buffer Power Control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPZA_A {
    #[doc = "0: DCDC IO buffer power down"]
    _0 = 0,
    #[doc = "1: DCDC IO buffer power up"]
    _1 = 1,
}
impl From<STOPZA_A> for bool {
    #[inline(always)]
    fn from(variant: STOPZA_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPZA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPZA_A {
        match self.bits {
            false => STOPZA_A::_0,
            true => STOPZA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOPZA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOPZA_A::_1
    }
}
#[doc = "Field `STOPZA` writer - DCDC IO Buffer Power Control bit"]
pub type STOPZA_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCDCCTL_SPEC, STOPZA_A, O>;
impl<'a, const O: u8> STOPZA_W<'a, O> {
    #[doc = "DCDC IO buffer power down"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPZA_A::_0)
    }
    #[doc = "DCDC IO buffer power up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPZA_A::_1)
    }
}
#[doc = "Field `LCBOOST` reader - LDO LCBOOST Mode Control bit"]
pub type LCBOOST_R = crate::BitReader<LCBOOST_A>;
#[doc = "LDO LCBOOST Mode Control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCBOOST_A {
    #[doc = "0: LDO power mode is other than LCBOOST"]
    _0 = 0,
    #[doc = "1: LDO power mode is in LCBOOST"]
    _1 = 1,
}
impl From<LCBOOST_A> for bool {
    #[inline(always)]
    fn from(variant: LCBOOST_A) -> Self {
        variant as u8 != 0
    }
}
impl LCBOOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCBOOST_A {
        match self.bits {
            false => LCBOOST_A::_0,
            true => LCBOOST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCBOOST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCBOOST_A::_1
    }
}
#[doc = "Field `LCBOOST` writer - LDO LCBOOST Mode Control bit"]
pub type LCBOOST_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCDCCTL_SPEC, LCBOOST_A, O>;
impl<'a, const O: u8> LCBOOST_W<'a, O> {
    #[doc = "LDO power mode is other than LCBOOST"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCBOOST_A::_0)
    }
    #[doc = "LDO power mode is in LCBOOST"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCBOOST_A::_1)
    }
}
#[doc = "Field `FST` reader - DCDC Fast Startup"]
pub type FST_R = crate::BitReader<FST_A>;
#[doc = "DCDC Fast Startup\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FST_A {
    #[doc = "0: Fast startupBecause it is a circuit-oriented expression, it is hard to understand. Reexamination of expression is necessary."]
    _0 = 0,
    #[doc = "1: Not fast startupBecause it is a circuit-oriented expression, it is hard to understand. Reexamination of expression is necessary."]
    _1 = 1,
}
impl From<FST_A> for bool {
    #[inline(always)]
    fn from(variant: FST_A) -> Self {
        variant as u8 != 0
    }
}
impl FST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FST_A {
        match self.bits {
            false => FST_A::_0,
            true => FST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FST_A::_1
    }
}
#[doc = "Field `FST` writer - DCDC Fast Startup"]
pub type FST_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCDCCTL_SPEC, FST_A, O>;
impl<'a, const O: u8> FST_W<'a, O> {
    #[doc = "Fast startupBecause it is a circuit-oriented expression, it is hard to understand. Reexamination of expression is necessary."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FST_A::_0)
    }
    #[doc = "Not fast startupBecause it is a circuit-oriented expression, it is hard to understand. Reexamination of expression is necessary."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FST_A::_1)
    }
}
#[doc = "Field `PD` reader - DCDC VREF Generate Disable bit"]
pub type PD_R = crate::BitReader<PD_A>;
#[doc = "DCDC VREF Generate Disable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD_A {
    #[doc = "0: DCDC VREF BIAS output enable"]
    _0 = 0,
    #[doc = "1: DCDC VREF BIAS output disable"]
    _1 = 1,
}
impl From<PD_A> for bool {
    #[inline(always)]
    fn from(variant: PD_A) -> Self {
        variant as u8 != 0
    }
}
impl PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_A {
        match self.bits {
            false => PD_A::_0,
            true => PD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PD_A::_1
    }
}
#[doc = "Field `PD` writer - DCDC VREF Generate Disable bit"]
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCDCCTL_SPEC, PD_A, O>;
impl<'a, const O: u8> PD_W<'a, O> {
    #[doc = "DCDC VREF BIAS output enable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PD_A::_0)
    }
    #[doc = "DCDC VREF BIAS output disable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - LDO/DCDC on/off Control bit"]
    #[inline(always)]
    pub fn dcdcon(&self) -> DCDCON_R {
        DCDCON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCDC OCP Function Enable bit"]
    #[inline(always)]
    pub fn ocpen(&self) -> OCPEN_R {
        OCPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - DCDC IO Buffer Power Control bit"]
    #[inline(always)]
    pub fn stopza(&self) -> STOPZA_R {
        STOPZA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LDO LCBOOST Mode Control bit"]
    #[inline(always)]
    pub fn lcboost(&self) -> LCBOOST_R {
        LCBOOST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DCDC Fast Startup"]
    #[inline(always)]
    pub fn fst(&self) -> FST_R {
        FST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DCDC VREF Generate Disable bit"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LDO/DCDC on/off Control bit"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcon(&mut self) -> DCDCON_W<0> {
        DCDCON_W::new(self)
    }
    #[doc = "Bit 1 - DCDC OCP Function Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ocpen(&mut self) -> OCPEN_W<1> {
        OCPEN_W::new(self)
    }
    #[doc = "Bit 4 - DCDC IO Buffer Power Control bit"]
    #[inline(always)]
    #[must_use]
    pub fn stopza(&mut self) -> STOPZA_W<4> {
        STOPZA_W::new(self)
    }
    #[doc = "Bit 5 - LDO LCBOOST Mode Control bit"]
    #[inline(always)]
    #[must_use]
    pub fn lcboost(&mut self) -> LCBOOST_W<5> {
        LCBOOST_W::new(self)
    }
    #[doc = "Bit 6 - DCDC Fast Startup"]
    #[inline(always)]
    #[must_use]
    pub fn fst(&mut self) -> FST_W<6> {
        FST_W::new(self)
    }
    #[doc = "Bit 7 - DCDC VREF Generate Disable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<7> {
        PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC/LDO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcctl](index.html) module"]
pub struct DCDCCTL_SPEC;
impl crate::RegisterSpec for DCDCCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dcdcctl::R](R) reader structure"]
impl crate::Readable for DCDCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdcctl::W](W) writer structure"]
impl crate::Writable for DCDCCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCCTL to value 0xc0"]
impl crate::Resettable for DCDCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
