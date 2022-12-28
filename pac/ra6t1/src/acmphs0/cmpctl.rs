#[doc = "Register `CMPCTL` reader"]
pub struct R(crate::R<CMPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPCTL` writer"]
pub struct W(crate::W<CMPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPCTL_SPEC>;
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
impl From<crate::W<CMPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CINV` reader - Comparator output polarity selection"]
pub type CINV_R = crate::BitReader<CINV_A>;
#[doc = "Comparator output polarity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINV_A {
    #[doc = "0: Comparator output not inverted"]
    _0 = 0,
    #[doc = "1: Comparator output inverted"]
    _1 = 1,
}
impl From<CINV_A> for bool {
    #[inline(always)]
    fn from(variant: CINV_A) -> Self {
        variant as u8 != 0
    }
}
impl CINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINV_A {
        match self.bits {
            false => CINV_A::_0,
            true => CINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINV_A::_1
    }
}
#[doc = "Field `CINV` writer - Comparator output polarity selection"]
pub type CINV_W<'a, const O: u8> = crate::BitWriter<'a, u8, CMPCTL_SPEC, CINV_A, O>;
impl<'a, const O: u8> CINV_W<'a, O> {
    #[doc = "Comparator output not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINV_A::_0)
    }
    #[doc = "Comparator output inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINV_A::_1)
    }
}
#[doc = "Field `COE` reader - Comparator output enable"]
pub type COE_R = crate::BitReader<COE_A>;
#[doc = "Comparator output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COE_A {
    #[doc = "0: Comparator output disabled (the output signal is low level)."]
    _0 = 0,
    #[doc = "1: Comparator output enabled"]
    _1 = 1,
}
impl From<COE_A> for bool {
    #[inline(always)]
    fn from(variant: COE_A) -> Self {
        variant as u8 != 0
    }
}
impl COE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COE_A {
        match self.bits {
            false => COE_A::_0,
            true => COE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COE_A::_1
    }
}
#[doc = "Field `COE` writer - Comparator output enable"]
pub type COE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CMPCTL_SPEC, COE_A, O>;
impl<'a, const O: u8> COE_W<'a, O> {
    #[doc = "Comparator output disabled (the output signal is low level)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COE_A::_0)
    }
    #[doc = "Comparator output enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COE_A::_1)
    }
}
#[doc = "Field `CSTEN` reader - Interrupt Select"]
pub type CSTEN_R = crate::BitReader<CSTEN_A>;
#[doc = "Interrupt Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTEN_A {
    #[doc = "0: Output via the Edge selector"]
    _0 = 0,
    #[doc = "1: Direct output"]
    _1 = 1,
}
impl From<CSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTEN_A {
        match self.bits {
            false => CSTEN_A::_0,
            true => CSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTEN_A::_1
    }
}
#[doc = "Field `CSTEN` writer - Interrupt Select"]
pub type CSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CMPCTL_SPEC, CSTEN_A, O>;
impl<'a, const O: u8> CSTEN_W<'a, O> {
    #[doc = "Output via the Edge selector"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSTEN_A::_0)
    }
    #[doc = "Direct output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTEN_A::_1)
    }
}
#[doc = "Field `CEG` reader - Selection of valid edge (Edge selector)"]
pub type CEG_R = crate::FieldReader<u8, CEG_A>;
#[doc = "Selection of valid edge (Edge selector)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CEG_A {
    #[doc = "0: No edge selection."]
    _00 = 0,
    #[doc = "1: Rising edge selection."]
    _01 = 1,
    #[doc = "2: Falling edge selection"]
    _10 = 2,
    #[doc = "3: Both-edge selection"]
    _11 = 3,
}
impl From<CEG_A> for u8 {
    #[inline(always)]
    fn from(variant: CEG_A) -> Self {
        variant as _
    }
}
impl CEG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEG_A {
        match self.bits {
            0 => CEG_A::_00,
            1 => CEG_A::_01,
            2 => CEG_A::_10,
            3 => CEG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CEG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CEG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CEG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CEG_A::_11
    }
}
#[doc = "Field `CEG` writer - Selection of valid edge (Edge selector)"]
pub type CEG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CMPCTL_SPEC, u8, CEG_A, 2, O>;
impl<'a, const O: u8> CEG_W<'a, O> {
    #[doc = "No edge selection."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CEG_A::_00)
    }
    #[doc = "Rising edge selection."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CEG_A::_01)
    }
    #[doc = "Falling edge selection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CEG_A::_10)
    }
    #[doc = "Both-edge selection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CEG_A::_11)
    }
}
#[doc = "Field `CDFS` reader - Noise filter selection"]
pub type CDFS_R = crate::FieldReader<u8, CDFS_A>;
#[doc = "Noise filter selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDFS_A {
    #[doc = "0: Noise filter not used."]
    _00 = 0,
    #[doc = "1: Noise filter sampling frequency is 2^3/PCLKB."]
    _01 = 1,
    #[doc = "2: Noise filter sampling frequency is 2^4/PCLKB."]
    _10 = 2,
    #[doc = "3: Noise filter sampling frequency is 2^5/PCLKB."]
    _11 = 3,
}
impl From<CDFS_A> for u8 {
    #[inline(always)]
    fn from(variant: CDFS_A) -> Self {
        variant as _
    }
}
impl CDFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDFS_A {
        match self.bits {
            0 => CDFS_A::_00,
            1 => CDFS_A::_01,
            2 => CDFS_A::_10,
            3 => CDFS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CDFS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CDFS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CDFS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CDFS_A::_11
    }
}
#[doc = "Field `CDFS` writer - Noise filter selection"]
pub type CDFS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CMPCTL_SPEC, u8, CDFS_A, 2, O>;
impl<'a, const O: u8> CDFS_W<'a, O> {
    #[doc = "Noise filter not used."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CDFS_A::_00)
    }
    #[doc = "Noise filter sampling frequency is 2^3/PCLKB."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CDFS_A::_01)
    }
    #[doc = "Noise filter sampling frequency is 2^4/PCLKB."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CDFS_A::_10)
    }
    #[doc = "Noise filter sampling frequency is 2^5/PCLKB."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CDFS_A::_11)
    }
}
#[doc = "Field `HCMPON` reader - Comparator operation control"]
pub type HCMPON_R = crate::BitReader<HCMPON_A>;
#[doc = "Comparator operation control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HCMPON_A {
    #[doc = "0: Operation stopped (the comparator outputs a low-level signal)"]
    _0 = 0,
    #[doc = "1: Operation enabled (input to the comparator pins is enabled"]
    _1 = 1,
}
impl From<HCMPON_A> for bool {
    #[inline(always)]
    fn from(variant: HCMPON_A) -> Self {
        variant as u8 != 0
    }
}
impl HCMPON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCMPON_A {
        match self.bits {
            false => HCMPON_A::_0,
            true => HCMPON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HCMPON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HCMPON_A::_1
    }
}
#[doc = "Field `HCMPON` writer - Comparator operation control"]
pub type HCMPON_W<'a, const O: u8> = crate::BitWriter<'a, u8, CMPCTL_SPEC, HCMPON_A, O>;
impl<'a, const O: u8> HCMPON_W<'a, O> {
    #[doc = "Operation stopped (the comparator outputs a low-level signal)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HCMPON_A::_0)
    }
    #[doc = "Operation enabled (input to the comparator pins is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HCMPON_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator output polarity selection"]
    #[inline(always)]
    pub fn cinv(&self) -> CINV_R {
        CINV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator output enable"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Select"]
    #[inline(always)]
    pub fn csten(&self) -> CSTEN_R {
        CSTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Selection of valid edge (Edge selector)"]
    #[inline(always)]
    pub fn ceg(&self) -> CEG_R {
        CEG_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bits 5:6 - Noise filter selection"]
    #[inline(always)]
    pub fn cdfs(&self) -> CDFS_R {
        CDFS_R::new((self.bits >> 5) & 3)
    }
    #[doc = "Bit 7 - Comparator operation control"]
    #[inline(always)]
    pub fn hcmpon(&self) -> HCMPON_R {
        HCMPON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator output polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn cinv(&mut self) -> CINV_W<0> {
        CINV_W::new(self)
    }
    #[doc = "Bit 1 - Comparator output enable"]
    #[inline(always)]
    #[must_use]
    pub fn coe(&mut self) -> COE_W<1> {
        COE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Select"]
    #[inline(always)]
    #[must_use]
    pub fn csten(&mut self) -> CSTEN_W<2> {
        CSTEN_W::new(self)
    }
    #[doc = "Bits 3:4 - Selection of valid edge (Edge selector)"]
    #[inline(always)]
    #[must_use]
    pub fn ceg(&mut self) -> CEG_W<3> {
        CEG_W::new(self)
    }
    #[doc = "Bits 5:6 - Noise filter selection"]
    #[inline(always)]
    #[must_use]
    pub fn cdfs(&mut self) -> CDFS_W<5> {
        CDFS_W::new(self)
    }
    #[doc = "Bit 7 - Comparator operation control"]
    #[inline(always)]
    #[must_use]
    pub fn hcmpon(&mut self) -> HCMPON_W<7> {
        HCMPON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpctl](index.html) module"]
pub struct CMPCTL_SPEC;
impl crate::RegisterSpec for CMPCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cmpctl::R](R) reader structure"]
impl crate::Readable for CMPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpctl::W](W) writer structure"]
impl crate::Writable for CMPCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPCTL to value 0"]
impl crate::Resettable for CMPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
